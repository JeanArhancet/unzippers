const fs = require('fs')
const path = require('path')

const test = require('ava')

const unziprs = require('../index')

test('unzip file with target', async (t) => {
  await unziprs.unzip('./resources/foo.zip', { target: './resources' })

  const readContent = fs.readFileSync('./resources/foo')
  t.deepEqual(readContent.toString(), 'Hello unziprs !')
})

test('unzip file without target', async (t) => {
  await unziprs.unzip('./resources/foo.zip')

  const readContent = fs.readFileSync('./resources/foo')
  t.deepEqual(readContent.toString(), 'Hello unziprs !')
})

test.after((t) => {
  fs.unlinkSync('./resources/foo')
})
