const fs = require('fs')
const path = require('path')

const test = require('ava')

const zippers = require('../index')

test('zip file without a target', async (t) => {
  await zippers.zip('./resources/foo')
  const existFooZip = fs.existsSync('./resources/foo.zip')
  t.is(existFooZip, true)
})

test('zip file with a target', async (t) => {
  await zippers.zip('./resources/foo', { target: './resources/bar.zip' })
  const existBarZip = fs.existsSync('./resources/bar.zip')
  t.is(existBarZip, true)
})

test('unzip file without a target', async (t) => {
  const oldFooStat = fs.statSync('./resources/foo')
  await zippers.zip('./resources/foo', { target: './resources/foobar.zip' })
  const existBarZip = fs.existsSync('./resources/foobar.zip')
  t.is(existBarZip, true)

  await zippers.unzip('./resources/foobar.zip')
  const newFooStat = fs.statSync('./resources/foo')

  t.is(oldFooStat.mtime !== newFooStat.mtime, true)

  const readContent = fs.readFileSync('./resources/foo')
  t.deepEqual(readContent.toString(), 'Hello Zippers !')
})

test('unzip file with a target', async (t) => {
  const oldFooStat = fs.statSync('./resources/foo')
  await zippers.zip('./resources/foo', { target: './resources/barfoo.zip' })
  const existBarZip = fs.existsSync('./resources/barfoo.zip')
  t.is(existBarZip, true)

  await zippers.unzip('./resources/barfoo.zip', { target: './resources' })
  const newFooStat = fs.statSync('./resources/foo')

  t.is(oldFooStat.mtime !== newFooStat.mtime, true)
  const readContent = fs.readFileSync('./resources/foo')
  t.deepEqual(readContent.toString(), 'Hello Zippers !')
})

test.after((t) => {
  fs.unlinkSync('./resources/foo.zip')
  fs.unlinkSync('./resources/bar.zip')
  fs.unlinkSync('./resources/foobar.zip')
  fs.unlinkSync('./resources/barfoo.zip')
})
