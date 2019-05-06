//! Direct Memory Access
//!
//! Adapted from the Adafruit ZeroDMA project:
//!
//! <https://github.com/adafruit/Adafruit_ZeroDMA>

// The MIT License (MIT)
//
// Copyright (c) 2016 Adafruit Industries
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

/// Status codes returned by some DMA functions and/or held in
/// a channel's jobStatus variable.
enum ZeroDMAstatus {
    DMA_STATUS_OK = 0,
    DMA_STATUS_ERR_NOT_FOUND,
    DMA_STATUS_ERR_NOT_INITIALIZED,
    DMA_STATUS_ERR_INVALID_ARG,
    DMA_STATUS_ERR_IO,
    DMA_STATUS_ERR_TIMEOUT,
    DMA_STATUS_BUSY,
    DMA_STATUS_SUSPEND,
    DMA_STATUS_ABORTED,
    DMA_STATUS_JOBSTATUS = -1 // For printStatus() function
};

class Adafruit_ZeroDMA {
  public:
    Adafruit_ZeroDMA(void);

    // DMA channel functions
    ZeroDMAstatus   allocate(void), // Allocates DMA channel
                    startJob(void),
                    free(void);     // Deallocates DMA channel
    void            trigger(void),
                    setTrigger(uint8_t trigger),
                    setAction(dma_transfer_trigger_action action),
                    setCallback(void (*callback)(Adafruit_ZeroDMA *) = NULL,
                    dma_callback_type type = DMA_CALLBACK_TRANSFER_DONE),
                    loop(boolean flag),
                    suspend(void),
                    resume(void),
                    abort(void),
                    setPriority(dma_priority pri),
                    printStatus(ZeroDMAstatus s = DMA_STATUS_JOBSTATUS);
    uint8_t         getChannel(void);

    // DMA descriptor functions
    DmacDescriptor *addDescriptor(void *src, void *dst, uint32_t count = 0,
                    dma_beat_size size = DMA_BEAT_SIZE_BYTE,
                    bool srcInc = true, bool dstInc = true,
                    uint32_t stepSize = DMA_ADDRESS_INCREMENT_STEP_SIZE_1,
                    bool stepSel = DMA_STEPSEL_DST);
    void            changeDescriptor(DmacDescriptor *d, void *src = NULL,
                    void *dst = NULL, uint32_t count = 0);

    void            _IRQhandler(uint8_t flags); // DO NOT TOUCH

    bool 			isActive();

  protected:
    uint8_t                     channel;
    volatile enum ZeroDMAstatus jobStatus;
    bool                        hasDescriptors;
    bool                        loopFlag;
    uint8_t                     peripheralTrigger;
    dma_transfer_trigger_action triggerAction;
    void                        (*callback[DMA_CALLBACK_N])(Adafruit_ZeroDMA *);
};

#ifdef DMAC_RESERVED_CHANNELS // SAMD core > 1.2.1
  #include <dma.h> // _descriptor[] and _writeback[] are extern'd here
  static volatile uint32_t _channelMask = DMAC_RESERVED_CHANNELS;
#else
  #include "utility/dma.h"
  static volatile uint32_t _channelMask = 0; // Bitmask of allocated channels

  // DMA descriptor list entry point (and writeback buffer) per channel
  __attribute__((__aligned__(16))) static DmacDescriptor // 128 bit alignment
    _descriptor[DMAC_CH_NUM] SECTION_DMAC_DESCRIPTOR,
    _writeback[DMAC_CH_NUM]  SECTION_DMAC_DESCRIPTOR;
#endif

// Pointer to ZeroDMA object for each channel is needed for the
// ISR (in C, outside of class context) to access callbacks.
static Adafruit_ZeroDMA *_dmaPtr[DMAC_CH_NUM] = {0}; // Init to NULL

// Adapted from ASF3 interrupt_sam_nvic.c:

static volatile unsigned long cpu_irq_critical_section_counter = 0;
static volatile unsigned char cpu_irq_prev_interrupt_state     = 0;

static void cpu_irq_enter_critical(void) {
    if(!cpu_irq_critical_section_counter) {
        if(__get_PRIMASK() == 0) { // IRQ enabled?
            __disable_irq();   // Disable it
            __DMB();
            cpu_irq_prev_interrupt_state = 1;
        } else {
            // Make sure the to save the prev state as false
            cpu_irq_prev_interrupt_state = 0;
        }

    }

    cpu_irq_critical_section_counter++;
}

static void cpu_irq_leave_critical(void) {
    // Check if the user is trying to leave a critical section
    // when not in a critical section
    if(cpu_irq_critical_section_counter > 0) {
        cpu_irq_critical_section_counter--;

        // Only enable global interrupts when the counter
        // reaches 0 and the state of the global interrupt flag
        // was enabled when entering critical state */
        if((!cpu_irq_critical_section_counter) &&
             cpu_irq_prev_interrupt_state) {
            __DMB();
            __enable_irq();
        }
    }
}

// CONSTRUCTOR -------------------------------------------------------------

// Constructor initializes Adafruit_ZeroDMA basics but does NOT allocate a
// DMA channel (that's done in allocate()) or start a job (that's done in
// startJob()).  This is because constructors in a global context are called
// before a sketch's setup() function, which may have some other hardware
// initialization of its own, don't want it clobbering us.
Adafruit_ZeroDMA::Adafruit_ZeroDMA(void) {
    channel           = 0xFF;  // Channel not yet allocated
    jobStatus         = DMA_STATUS_OK;
    hasDescriptors    = false; // No descriptors allocated yet
    loopFlag          = false;
    peripheralTrigger = 0;     // Software trigger only by default
    triggerAction     = DMA_TRIGGER_ACTON_TRANSACTION;
    memset(callback, 0, sizeof(callback));
}

// TODO: add destructor? Should stop job, delete descriptors, free channel.

// INTERRUPT SERVICE ROUTINE -----------------------------------------------

// This is a C function that exists outside the Adafruit_ZeroDMA context.
// DMA channel number is determined from the INTPEND register, from this
// we get a ZeroDMA object pointer through the _dmaPtr[] array.
// (It's done this way because jobStatus and callback[] are protected
// elements in the ZeroDMA object -- we can't touch them in C, but the
// next function after this, being part of the ZeroDMA class, can.)

#ifdef __SAMD51__
void DMAC_0_Handler(void) {
#else
void DMAC_Handler(void) {
#endif
    cpu_irq_enter_critical();

    uint8_t channel = DMAC->INTPEND.bit.ID; // Channel # causing interrupt
    if(channel < DMAC_CH_NUM) {
        Adafruit_ZeroDMA *dma;
        if((dma = _dmaPtr[channel])) { // -> Channel's ZeroDMA object
#ifdef __SAMD51__
            // Call IRQ handler with channel #
            dma->_IRQhandler(channel);
#else
            DMAC->CHID.bit.ID = channel;
            // Call IRQ handler with interrupt flag(s)
            dma->_IRQhandler(DMAC->CHINTFLAG.reg);
#endif
        }
    }

    cpu_irq_leave_critical();
}

#ifdef __SAMD51__
void DMAC_1_Handler(void) __attribute__((weak, alias("DMAC_0_Handler")));
void DMAC_2_Handler(void) __attribute__((weak, alias("DMAC_0_Handler")));
void DMAC_3_Handler(void) __attribute__((weak, alias("DMAC_0_Handler")));
void DMAC_4_Handler(void) __attribute__((weak, alias("DMAC_0_Handler")));
#endif

void Adafruit_ZeroDMA::_IRQhandler(uint8_t flags) {
#ifdef __SAMD51__
    // 'flags' is initially passed in as channel number,
    // from which we look up the actual interrupt flags...
    flags = DMAC->Channel[flags].CHINTFLAG.reg;
#endif
    if(flags & DMAC_CHINTENCLR_TERR) {
        // Clear error flag
#ifdef __SAMD51__
        DMAC->Channel[channel].CHINTFLAG.reg = DMAC_CHINTENCLR_TERR;
#else
        DMAC->CHINTFLAG.reg = DMAC_CHINTENCLR_TERR;
#endif
        jobStatus           = DMA_STATUS_ERR_IO;
        if(callback[DMA_CALLBACK_TRANSFER_ERROR])
            callback[DMA_CALLBACK_TRANSFER_ERROR](this);
    } else if(flags & DMAC_CHINTENCLR_TCMPL) {
        // Clear transfer complete flag
#ifdef __SAMD51__
        DMAC->Channel[channel].CHINTFLAG.reg = DMAC_CHINTENCLR_TCMPL;
#else
        DMAC->CHINTFLAG.reg = DMAC_CHINTENCLR_TCMPL;
#endif
        jobStatus           = DMA_STATUS_OK;
        if(callback[DMA_CALLBACK_TRANSFER_DONE])
            callback[DMA_CALLBACK_TRANSFER_DONE](this);
    } else if(flags & DMAC_CHINTENCLR_SUSP) {
        // Clear channel suspend flag
#ifdef __SAMD51__
        DMAC->Channel[channel].CHINTFLAG.reg = DMAC_CHINTENCLR_SUSP;
#else
        DMAC->CHINTFLAG.reg = DMAC_CHINTENCLR_SUSP;
#endif
        jobStatus           = DMA_STATUS_SUSPEND;
        if(callback[DMA_CALLBACK_CHANNEL_SUSPEND])
            callback[DMA_CALLBACK_CHANNEL_SUSPEND](this);
    }
}

// DMA CHANNEL FUNCTIONS ---------------------------------------------------

// Allocates channel for ZeroDMA object
ZeroDMAstatus Adafruit_ZeroDMA::allocate(void) {

    if(channel < DMAC_CH_NUM) return DMA_STATUS_OK; // Already alloc'd!

    // Find index of first free DMA channel.  As currently written,
    // this "does not play well with others" as it assumes _channelMask
    // is the final arbiter of channels in use (this is true only within
    // this library -- but other DMA-driven code may have allocated its
    // own channel(s) elsewhere, sometimes with an equally broken
    // approach).  A possible alternate approach, I haven't tested this
    // yet, might be to loop through each channel, set DMAC->CHID.bit.ID
    // and then test whether CHCTRLA.bit.ENABLE is set?  But for now...
    for(channel=0; (channel < DMAC_CH_NUM) &&
      (_channelMask & (1 << channel)); channel++);
    // Doesn't help that code later does a software reset of the DMA
    // controller, which would blow out other DMA-using libraries
    // anyway (or they're just as likely to blow out this one).
    // I think it's just an all-or-nothing affair...use one library
    // for DMA everything, never mix and match.

    if(channel >= DMAC_CH_NUM) // No free channel!
        return DMA_STATUS_ERR_NOT_FOUND;

    cpu_irq_enter_critical();

    if(!_channelMask) { // No channels allocated yet; initialize DMA!
#if !defined(DMAC_RESERVED_CHANNELS)
#if (SAML21) || (SAML22) || (SAMC20) || (SAMC21)
        PM->AHBMASK.bit.DMAC_       = 1;
#elif defined(__SAMD51__)
        MCLK->AHBMASK.bit.DMAC_     = 1; // Initialize DMA clocks
#else
        PM->AHBMASK.bit.DMAC_       = 1; // Initialize DMA clocks
        PM->APBBMASK.bit.DMAC_      = 1;
#endif
        DMAC->CTRL.bit.DMAENABLE    = 0; // Disable DMA controller
        DMAC->CTRL.bit.SWRST        = 1; // Perform software reset

        // Initialize descriptor list addresses
        DMAC->BASEADDR.bit.BASEADDR = (uint32_t)_descriptor;
        DMAC->WRBADDR.bit.WRBADDR   = (uint32_t)_writeback;
        memset(_descriptor, 0, sizeof(_descriptor));
        memset(_writeback , 0, sizeof(_writeback));

        // Re-enable DMA controller with all priority levels
        DMAC->CTRL.reg = DMAC_CTRL_DMAENABLE | DMAC_CTRL_LVLEN(0xF);
#endif

        // Enable DMA interrupt at lowest priority
#ifdef __SAMD51__
        IRQn_Type irqs[] = { DMAC_0_IRQn, DMAC_1_IRQn, DMAC_2_IRQn,
          DMAC_3_IRQn, DMAC_4_IRQn };
        for(uint8_t i=0; i<(sizeof irqs / sizeof irqs[0]); i++) {
            NVIC_EnableIRQ(irqs[i]);
            NVIC_SetPriority(irqs[i], (1<<__NVIC_PRIO_BITS)-1);
        }
#else
        NVIC_EnableIRQ(DMAC_IRQn);
        NVIC_SetPriority(DMAC_IRQn, (1 << __NVIC_PRIO_BITS) - 1);
#endif
    }

    _channelMask    |= 1 << channel; // Mark channel as allocated
    _dmaPtr[channel] = this;         // Channel-index-to-object pointer

    // Reset the allocated channel
#ifdef __SAMD51__
    DMAC->Channel[channel].CHCTRLA.bit.ENABLE  = 0;
    DMAC->Channel[channel].CHCTRLA.bit.SWRST   = 1;
#else
    DMAC->CHID.bit.ID         = channel;
    DMAC->CHCTRLA.bit.ENABLE  = 0;
    DMAC->CHCTRLA.bit.SWRST   = 1;
#endif

    // Clear software trigger
    DMAC->SWTRIGCTRL.reg     &= ~(1 << channel);

    // Configure default behaviors
#ifdef __SAMD51__
    DMAC->Channel[channel].CHPRILVL.bit.PRILVL = 0;
    DMAC->Channel[channel].CHCTRLA.bit.TRIGSRC = peripheralTrigger;
    DMAC->Channel[channel].CHCTRLA.bit.TRIGACT = triggerAction;
    DMAC->Channel[channel].CHCTRLA.bit.BURSTLEN =
      DMAC_CHCTRLA_BURSTLEN_SINGLE_Val; // Single-beat burst length
#else
    DMAC->CHCTRLB.bit.LVL     = 0;
    DMAC->CHCTRLB.bit.TRIGSRC = peripheralTrigger;
    DMAC->CHCTRLB.bit.TRIGACT = triggerAction;
#endif

    cpu_irq_leave_critical();

    return DMA_STATUS_OK;
}

void Adafruit_ZeroDMA::setPriority(dma_priority pri) {
#ifdef __SAMD51__
    DMAC->Channel[channel].CHPRILVL.bit.PRILVL = pri;
#else
    DMAC->CHCTRLB.bit.LVL = pri;
#endif
}

// Deallocate DMA channel
// TODO: should this delete/deallocate the descriptor list?
ZeroDMAstatus Adafruit_ZeroDMA::free(void) {

    ZeroDMAstatus status = DMA_STATUS_OK;

    cpu_irq_enter_critical(); // jobStatus is volatile

        if(jobStatus == DMA_STATUS_BUSY) {
        status = DMA_STATUS_BUSY; // Can't leave when busy
    } else if((channel < DMAC_CH_NUM) && (_channelMask & (1 << channel))) {
        // Valid in-use channel; release it
        _channelMask &= ~(1 << channel); // Clear bit
        if(!_channelMask) {              // No more channels in use?
#ifdef __SAMD51__
            NVIC_DisableIRQ(DMAC_0_IRQn); // Disable DMA interrupt
            DMAC->CTRL.bit.DMAENABLE = 0; // Disable DMA
            MCLK->AHBMASK.bit.DMAC_  = 0; // Disable DMA clock
#else
            NVIC_DisableIRQ(DMAC_IRQn);   // Disable DMA interrupt
            DMAC->CTRL.bit.DMAENABLE = 0; // Disable DMA
            PM->APBBMASK.bit.DMAC_   = 0; // Disable DMA clocks
            PM->AHBMASK.bit.DMAC_    = 0;
#endif
        }
        _dmaPtr[channel] = NULL;
        channel          = 0xFF;
    } else {
        status = DMA_STATUS_ERR_NOT_INITIALIZED; // Channel not in use
    }

    cpu_irq_leave_critical();

    return status;
}

// Start DMA transfer job.  Channel and descriptors should be allocated
// before calling this.
ZeroDMAstatus Adafruit_ZeroDMA::startJob(void) {
    ZeroDMAstatus status = DMA_STATUS_OK;

    cpu_irq_enter_critical(); // Job status is volatile

    if(jobStatus == DMA_STATUS_BUSY) {
        status = DMA_STATUS_BUSY; // Resource is busy
    } else if(channel >= DMAC_CH_NUM) {
        status = DMA_STATUS_ERR_NOT_INITIALIZED; // Channel not in use
    } else if(!hasDescriptors || (_descriptor[channel].BTCNT.reg <= 0)) {
        status = DMA_STATUS_ERR_INVALID_ARG; // Bad transfer size
    } else {
        uint8_t i, interruptMask = 0;
        for(i=0; i<DMA_CALLBACK_N; i++)
            if(callback[i]) interruptMask |= (1 << i);
        jobStatus            = DMA_STATUS_BUSY;
#ifdef __SAMD51__
        DMAC->Channel[channel].CHINTENSET.reg =
          DMAC_CHINTENSET_MASK &  interruptMask;
        DMAC->Channel[channel].CHINTENCLR.reg =
          DMAC_CHINTENCLR_MASK & ~interruptMask;
        DMAC->Channel[channel].CHCTRLA.bit.ENABLE = 1;
#else
        DMAC->CHID.bit.ID    = channel;
        DMAC->CHINTENSET.reg = DMAC_CHINTENSET_MASK &  interruptMask;
        DMAC->CHINTENCLR.reg = DMAC_CHINTENCLR_MASK & ~interruptMask;
        DMAC->CHCTRLA.bit.ENABLE = 1; // Enable the transfer channel
#endif
    }

    cpu_irq_leave_critical();

    return status;
}

// Set and enable callback function for ZeroDMA object. This can be called
// before or after channel and/or descriptors are allocated, but needs
// to be called before job is started.
void Adafruit_ZeroDMA::setCallback(
  void (*cb)(Adafruit_ZeroDMA *), dma_callback_type type) {
    callback[type] = cb;
}

// Suspend/resume don't quite do what I thought -- avoid using for now.
void Adafruit_ZeroDMA::suspend(void) {
    cpu_irq_enter_critical();
#ifdef __SAMD51__
    DMAC->Channel[channel].CHCTRLB.reg |= DMAC_CHCTRLB_CMD_SUSPEND;
#else
    DMAC->CHID.bit.ID  = channel;
    DMAC->CHCTRLB.reg |= DMAC_CHCTRLB_CMD_SUSPEND;
#endif
    cpu_irq_leave_critical();
}

#define MAX_JOB_RESUME_COUNT 10000
void Adafruit_ZeroDMA::resume(void) {
    cpu_irq_enter_critical(); // jobStatus is volatile
    if(jobStatus == DMA_STATUS_SUSPEND) {
        int      count;
        uint32_t bitMask   = 1 << channel;
#ifdef __SAMD51__
        DMAC->Channel[channel].CHCTRLB.reg |= DMAC_CHCTRLB_CMD_RESUME;
#else
        DMAC->CHID.bit.ID  = channel;
        DMAC->CHCTRLB.reg |= DMAC_CHCTRLB_CMD_RESUME;
#endif

        for(count = 0; (count < MAX_JOB_RESUME_COUNT) &&
          !(DMAC->BUSYCH.reg & bitMask); count++);

        jobStatus = (count < MAX_JOB_RESUME_COUNT) ?
          DMA_STATUS_BUSY : DMA_STATUS_ERR_TIMEOUT;
    }
    cpu_irq_leave_critical();
}

// Abort is OK though.
void Adafruit_ZeroDMA::abort(void) {
    if(channel <= DMAC_CH_NUM) {
        cpu_irq_enter_critical();
#ifdef __SAMD51__
        DMAC->Channel[channel].CHCTRLA.reg = 0; // Disable channel
#else
        DMAC->CHID.bit.ID = channel; // Select channel
        DMAC->CHCTRLA.reg = 0;       // Disable
#endif
        jobStatus         = DMA_STATUS_ABORTED;
        cpu_irq_leave_critical();
    }
}

// Set DMA peripheral trigger.
// This can be done before or after channel is allocated.
void Adafruit_ZeroDMA::setTrigger(uint8_t trigger) {
    peripheralTrigger = trigger; // Save value for allocate()

    // If channel already allocated, configure peripheral trigger
    // (old lib required configure before alloc -- either way OK now)
    if(channel < DMAC_CH_NUM) {
        cpu_irq_enter_critical();
#ifdef __SAMD51__
        DMAC->Channel[channel].CHCTRLA.bit.TRIGSRC = trigger;
#else
        DMAC->CHID.bit.ID         = channel;
        DMAC->CHCTRLB.bit.TRIGSRC = trigger;
#endif
        cpu_irq_leave_critical();
    }
}

// Set DMA trigger action.
// This can be done before or after channel is allocated.
void Adafruit_ZeroDMA::setAction(dma_transfer_trigger_action action) {
    triggerAction = action; // Save value for allocate()

    // If channel already allocated, configure trigger action
    // (old lib required configure before alloc -- either way OK now)
    if(channel < DMAC_CH_NUM) {
        cpu_irq_enter_critical();
#ifdef __SAMD51__
        DMAC->Channel[channel].CHCTRLA.bit.TRIGACT = action;
#else
        DMAC->CHID.bit.ID         = channel;
        DMAC->CHCTRLB.bit.TRIGACT = action;
#endif
        cpu_irq_leave_critical();
    }
}

// Issue software trigger. Channel must be allocated & descriptors added!
void Adafruit_ZeroDMA::trigger(void) {
    if((channel <= DMAC_CH_NUM) & hasDescriptors)
        DMAC->SWTRIGCTRL.reg |= (1 << channel);
}

uint8_t Adafruit_ZeroDMA::getChannel(void) {
    return channel;
}

// DMA DESCRIPTOR FUNCTIONS ------------------------------------------------

// Allocates a new DMA descriptor (if needed) and appends it to the
// channel's descriptor list.  Returns pointer to DmacDescriptor,
// or NULL on various errors.  You'll want to keep the pointer for
// later if you need to modify or free the descriptor.
// Channel must be allocated first!
DmacDescriptor *Adafruit_ZeroDMA::addDescriptor(
  void           *src,
  void           *dst,
  uint32_t        count,
  dma_beat_size   size,
  bool            srcInc,
  bool            dstInc,
  uint32_t        stepSize,
  bool            stepSel) {

    // Channel must be allocated first
        if(channel >= DMAC_CH_NUM) return NULL;

    // Can't do while job's busy
    if(jobStatus == DMA_STATUS_BUSY) return NULL;

    DmacDescriptor *desc;

    // Scan descriptor list to find last entry.  If an entry's
    // DESCADDR value is 0, that's the end of the list and it's
    // currently un-looped.  If the DESCADDR value is the same
    // as the first entry, that's the end of the list and it's
    // looped.  Either way, set the last entry's DESCADDR value
    // to the new descriptor, and the descriptor's own DESCADDR
    // will be set later either to 0 or the list head.
    if(hasDescriptors) {
        // DMA descriptors must be 128-bit (16 byte) aligned.
        // memalign() is considered 'obsolete' but it's replacements
        // (aligned_alloc() or posix_memalign()) are not currently
        // available in the version of ARM GCC in use, but this is,
        // so here we are.
        if(!(desc = (DmacDescriptor *)memalign(16,
          sizeof(DmacDescriptor))))
            return NULL;
        DmacDescriptor *prev = &_descriptor[channel];
        while(prev->DESCADDR.reg &&
             (prev->DESCADDR.reg != (uint32_t)&_descriptor[channel])) {
            prev = (DmacDescriptor *)prev->DESCADDR.reg;
        }
        prev->DESCADDR.reg = (uint32_t)desc;
    } else {
        desc = &_descriptor[channel];
    }
    hasDescriptors = true;

    uint8_t bytesPerBeat; // Beat transfer size IN BYTES
    switch(size) {
       default:                  bytesPerBeat = 1; break;
       case DMA_BEAT_SIZE_HWORD: bytesPerBeat = 2; break;
       case DMA_BEAT_SIZE_WORD:  bytesPerBeat = 4; break;
    }

    desc->BTCTRL.bit.VALID    = true;
    desc->BTCTRL.bit.EVOSEL   = DMA_EVENT_OUTPUT_DISABLE;
    desc->BTCTRL.bit.BLOCKACT = DMA_BLOCK_ACTION_NOACT;
    desc->BTCTRL.bit.BEATSIZE = size;
    desc->BTCTRL.bit.SRCINC   = srcInc;
    desc->BTCTRL.bit.DSTINC   = dstInc;
    desc->BTCTRL.bit.STEPSEL  = stepSel;
    desc->BTCTRL.bit.STEPSIZE = stepSize;
    desc->BTCNT.reg           = count;
    desc->SRCADDR.reg         = (uint32_t)src;

    if(srcInc){
        if(stepSel) {
            desc->SRCADDR.reg +=
              bytesPerBeat * count * (1 << stepSize);
        } else {
            desc->SRCADDR.reg += bytesPerBeat * count;
        }
    }

    desc->DSTADDR.reg         = (uint32_t)dst;

    if(dstInc){
        if(!stepSel) {
            desc->DSTADDR.reg +=
              bytesPerBeat * count * (1 << stepSize);
        } else {
            desc->DSTADDR.reg += bytesPerBeat * count;
        }
    }

    desc->DESCADDR.reg = loopFlag ? (uint32_t)&_descriptor[channel] : 0;

    return desc;
}

// Modify DMA descriptor with a new source address, destination address &
// block transfer count.  All other attributes (including increment enables,
// etc.) are unchanged.  Mostly for changing the data being pushed to a
// peripheral (DAC, SPI, whatev.)
void Adafruit_ZeroDMA::changeDescriptor(DmacDescriptor *desc,
  void *src, void *dst, uint32_t count) {

    uint8_t bytesPerBeat; // Beat transfer size IN BYTES
    switch(desc->BTCTRL.bit.BEATSIZE) {
       default:                  bytesPerBeat = 1; break;
       case DMA_BEAT_SIZE_HWORD: bytesPerBeat = 2; break;
       case DMA_BEAT_SIZE_WORD:  bytesPerBeat = 4; break;
    }

    if(count) desc->BTCNT.reg = count;

    if(src) {
        desc->SRCADDR.reg = (uint32_t)src;
        if(desc->BTCTRL.bit.SRCINC){
            if(desc->BTCTRL.bit.STEPSEL) {
              desc->SRCADDR.reg += desc->BTCNT.reg *
                bytesPerBeat * (1 << desc->BTCTRL.bit.STEPSIZE);
            } else {
              desc->SRCADDR.reg += desc->BTCNT.reg * bytesPerBeat;
            }
        }
    }

    if(dst) {
        desc->DSTADDR.reg = (uint32_t)dst;
        if(desc->BTCTRL.bit.DSTINC){
            if(!desc->BTCTRL.bit.STEPSEL) {
              desc->DSTADDR.reg += desc->BTCNT.reg *
                bytesPerBeat * (1 << desc->BTCTRL.bit.STEPSIZE);
            } else {
              desc->DSTADDR.reg += desc->BTCNT.reg * bytesPerBeat;
            }
        }
    }

// I think this code is here by accident -- disabling for now.
#if 0
    cpu_irq_enter_critical();
    jobStatus          = DMA_STATUS_OK;
#ifdef __SAMD51__
    DMAC->Channel[channel].CHCTRLA.bit.ENABLE = 1;
#else
    DMAC->CHID.bit.ID  = channel;
    DMAC->CHCTRLA.reg |= DMAC_CHCTRLA_ENABLE;
#endif
    cpu_irq_leave_critical();
#endif
}

// TODO: delete descriptor, delete whole descriptor chain

// Select whether channel's descriptor list should repeat or not.
// This can be done before or after channel & any descriptors are allocated.
void Adafruit_ZeroDMA::loop(boolean flag) {
    // The loop selection is 'sticky' -- that is, you can enable or
    // disable looping before a descriptor list is built, or after
    // the fact.  This requires some extra steps in the library code
    // but avoids a must-do-in-X-order constraint on user.
    loopFlag = flag;

    if(hasDescriptors) { // Descriptor list already started?
        // Scan descriptor list to find last entry.  If an entry's
        // DESCADDR value is 0, that's the end of the list and it's
        // currently un-looped.  If the DESCADDR value is the same
        // as the first entry, that's the end of the list and it's
        // already looped.
        DmacDescriptor *desc = &_descriptor[channel];
        while(desc->DESCADDR.reg &&
             (desc->DESCADDR.reg != (uint32_t)&_descriptor[channel])) {
            desc = (DmacDescriptor *)desc->DESCADDR.reg;
        }
        // Loop or unloop descriptor list as appropriate
        desc->DESCADDR.reg = loopFlag ?
          (uint32_t)&_descriptor[channel] : 0;
    }
}

// MISCELLANY --------------------------------------------------------------

void Adafruit_ZeroDMA::printStatus(ZeroDMAstatus s) {
    if(s == DMA_STATUS_JOBSTATUS) s = jobStatus;
    Serial.print("Status: ");
    switch(s) {
       case DMA_STATUS_OK:
        Serial.println("OK");
        break;
       case DMA_STATUS_ERR_NOT_FOUND:
        Serial.println("NOT FOUND");
        break;
       case DMA_STATUS_ERR_NOT_INITIALIZED:
        Serial.println("NOT INITIALIZED");
        break;
       case DMA_STATUS_ERR_INVALID_ARG:
        Serial.println("INVALID ARGUMENT");
        break;
       case DMA_STATUS_ERR_IO:
        Serial.println("IO ERROR");
        break;
       case DMA_STATUS_ERR_TIMEOUT:
        Serial.println("TIMEOUT");
        break;
       case DMA_STATUS_BUSY:
        Serial.println("BUSY");
        break;
       case DMA_STATUS_SUSPEND:
        Serial.println("SUSPENDED");
        break;
       case DMA_STATUS_ABORTED:
        Serial.println("ABORTED");
        break;
       default:
        Serial.print("Unknown 0x");
        Serial.println((int)s);
        break;
    }
}

bool Adafruit_ZeroDMA::isActive(){
    return _writeback[channel].BTCTRL.bit.VALID;
}
