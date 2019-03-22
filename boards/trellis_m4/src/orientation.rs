//! Orientation tracking support for the NeoTrellis M4
//!
//! This module provides tablet-like orientation tracking. It's designed to
//! work when the device is being handheld, in a "portrait" orientation,
//! and attempts to detect whether the device's plugs (USB, audio, etc)
//! are pointed up or down.

pub use adxl343::Ax3;

use crate::Adxl343;
use core::ops::{Index, IndexMut};
use hal::sercom::I2CError;

/// Number of samples used to guess the device's orientation
const NUM_SAMPLES: usize = 32;

/// Denominator to normalize acceleration rates between 0.0 and 1.0
const ACCEL_NORMALIZE: f32 = 256.0;

/// Orientation of the NeoTrellis M4, as detected by the onboard ADXL343
/// accelerometer.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Orientation {
    /// Device's plugs (USB, audio, etc) are pointed up
    Up,

    /// Device's plugs (USB, audio, etc) are pointed down
    Down,

    /// Unable to determine device's orientation from the current data
    Unknown,
}

/// Orientation-tracking support: computes a moving average of accelerometer
/// data from which the device's orientation is guessed
pub struct Tracker {
    /// The underlying accelerometer device
    accel: Adxl343,

    /// Samples of x-axis accelerometer data
    x_samples: Samples,

    /// Samples of y-axis accelerometer data
    y_samples: Samples,

    /// Samples of z-axis accelerometer data
    z_samples: Samples,

    /// Previously computed orientations
    estimates: [f32; NUM_SAMPLES],

    /// Current position within our sample data array
    position: usize,
}

impl Tracker {
    /// Create a new orientation tracker from the given Adxl343 accelerometer
    pub fn new(accel: Adxl343) -> Self {
        Self {
            accel,
            x_samples: Samples::default(),
            y_samples: Samples::default(),
            z_samples: Samples::default(),
            estimates: [0.0; NUM_SAMPLES],
            position: 0,
        }
    }

    /// Get a moving average of acceleration readings from the underlying
    /// `Adxl343` device
    pub fn accel_avg(&mut self) -> Result<Ax3, I2CError> {
        Ok(Ax3 {
            x: self.x_samples.mean(),
            y: self.y_samples.mean(),
            z: self.z_samples.mean(),
        })
    }

    /// Compute the average direction the device is pointing. This is roughly
    /// the angle the device is offset from the vertical axis (-1 .. 1)
    pub fn direction_avg(&self) -> f32 {
        self.estimates.iter().sum::<f32>() / NUM_SAMPLES as f32
    }

    /// Guess the device's orientation based on a moving average of previous
    /// accelerometer data (taking a new sample)
    pub fn orientation(&mut self) -> Result<Orientation, I2CError> {
        self.update()?;
        let n = self.direction_avg();

        let result = if n > 0.1 {
            Orientation::Up
        } else if n < -0.1 {
            Orientation::Down
        } else {
            Orientation::Unknown
        };

        Ok(result)
    }

    /// Borrow the current x, y, and z sample buffers
    pub fn samples(&self) -> (&Samples, &Samples, &Samples) {
        (&self.x_samples, &self.y_samples, &self.z_samples)
    }

    /// Take an accelerometer reading and store it in the internal buffer
    pub fn update(&mut self) -> Result<(), I2CError> {
        let accel = self.accel.accel()?;

        self.x_samples[self.position] = accel.x / ACCEL_NORMALIZE;
        self.y_samples[self.position] = accel.y / ACCEL_NORMALIZE;
        self.z_samples[self.position] = accel.z / ACCEL_NORMALIZE;

        let x_corrected = self.x_samples.corrected_mean();
        let y_corrected = self.y_samples.corrected_mean();

        self.estimates[self.position] = x_corrected - y_corrected;
        self.position = (self.position + 1) % NUM_SAMPLES;

        Ok(())
    }
}

impl From<Adxl343> for Tracker {
    fn from(accel: Adxl343) -> Tracker {
        Tracker::new(accel)
    }
}

/// Buffer of samples for a given coordinate
#[derive(Default)]
pub struct Samples([f32; NUM_SAMPLES]);

impl Samples {
    /// Compute arithmetic mean
    pub fn mean(&self) -> f32 {
        self.0.iter().sum::<f32>() / NUM_SAMPLES as f32
    }

    /// Compute arithmetic mean and variance simultaneously
    pub fn mean_and_variance(&self) -> (f32, f32) {
        let mean = self.mean();

        let variance = self
            .0
            .iter()
            .fold(0.0, |sum, n| sum + (*n - mean) * (*n - mean))
            / (NUM_SAMPLES as f32 - 1.0);

        (mean, variance)
    }

    /// Compute a "corrected" mean weighted by the variance
    pub fn corrected_mean(&self) -> f32 {
        let (mean, variance) = self.mean_and_variance();

        let weight = if variance < 0.333 {
            1.0 - (3.0 * variance)
        } else {
            0.0
        };

        weight * mean
    }
}

impl Index<usize> for Samples {
    type Output = f32;

    fn index(&self, index: usize) -> &f32 {
        &self.0[index]
    }
}

impl IndexMut<usize> for Samples {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        self.0.index_mut(index)
    }
}
