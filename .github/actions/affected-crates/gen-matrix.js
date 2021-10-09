const path = require('path')
const findUp = require('find-up')
const {exists} = findUp

async function getCrate(dir){
  return await findUp(async dir =>{
    const hasCargoFile = await exists(path.join(dir, 'Cargo.toml'))
    return hasCargoFile && dir
  }, {type: 'directory', cwd: dir})
}

async function getAffectedCrates (files) {
  let affectedCrates = []

  for(let file of files){
    console.log('testing affected file as a crate:', file)
    let {dir} = path.parse(path.resolve(file))
    let affectedCrate = await getCrate(dir)
    affectedCrates.push(affectedCrate)
  }

  return affectedCrates.filter(x => x !== undefined)
}

async function generateBuildMatrix(files) {
  const affectedCrates = [...new Set(await getAffectedCrates(files))]
  console.log('affected crates: ', affectedCrates)
  return affectedCrates.length ? { crate: affectedCrates, toolchain: ['nightly', 'stable'] } : null
}

exports.generateBuildMatrix = generateBuildMatrix
exports.getCrate = getCrate
