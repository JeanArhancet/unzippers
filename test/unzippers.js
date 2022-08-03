const fs = require('fs')
const path = require('path')

const test = require('ava')

const unzippers = require('../index')

test('unzip file with target', async (t) => {
  await unzippers.unzip('./resources/foo.zip', { target: './resources' })

  const readContent = fs.readFileSync('./resources/foo')
  t.deepEqual(readContent.toString(), 'Hello unzippers !')
})

test('unzip file without target', async (t) => {
  await unzippers.unzip('./resources/foo.zip')

  const readContent = fs.readFileSync('./resources/foo')
  t.deepEqual(readContent.toString(), 'Hello unzippers !')
})

test.after((t) => {
  fs.unlinkSync('./resources/foo')
})
