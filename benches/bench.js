const fs = require('fs')
const { promisify } = require('util')

const AdmZip = require('adm-zip')
const { add, suite, cycle, complete } = require('benny')
const extract = require('extract-zip')
const glob = require('glob')
const StreamZip = require('node-stream-zip')

const unzippers = require('../index')

const filesToUnzip = function () {
  return new Promise((resolve, reject) => {
    glob('*.zip', { cwd: __dirname, absolute: true }, function (err, files) {
      if (err) reject(err)
      resolve(files)
    })
  })
}

async function unzip() {
  const files = await filesToUnzip()
  for await (const file of files) {
    const stats = fs.statSync(file)
    const fileSizeInMegabytes = stats.size / (1024 * 1024)
    await suite(
      `Unzip with file ${fileSizeInMegabytes} MB`,
      add(`extract-zip with file ${fileSizeInMegabytes} MB`, async () => {
        await extract(file, { dir: __dirname })
      }),
      add(`node-stream-zip with file ${fileSizeInMegabytes} MB`, async () => {
        const zip = new StreamZip.async({ file: file })
        await zip.extract(null, __dirname)
        await zip.close()
      }),
      add(`unzippers with file ${fileSizeInMegabytes} MB`, async () => {
        await unzippers.unzip(file)
      }),
      add(`adm-zip with file ${fileSizeInMegabytes} MB`, async () => {
        // reading archives
        const zip = new AdmZip(file)

        await promisify(zip.extractAllToAsync)(__dirname, true, (err) => {})
      }),
      cycle(),
      complete(),
    )
  }
}

unzip().catch((e) => {
  console.error('Error to unzip', e)
})
