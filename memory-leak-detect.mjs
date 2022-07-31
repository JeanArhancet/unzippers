import path from 'node:path'
import { fileURLToPath } from 'node:url'

import chalk from 'chalk'
import prettyBytes from 'pretty-bytes'
import { table } from 'table'

import { zip, unzip } from './index.js'

const initial = process.memoryUsage()
const __filename = fileURLToPath(import.meta.url)
const __dirname = path.dirname(__filename)

function displayMemoryUsageFromNode(initialMemoryUsage) {
  const finalMemoryUsage = process.memoryUsage()
  const titles = Object.keys(initialMemoryUsage).map((k) => chalk.whiteBright(k))
  const tableData = [titles]
  const diffColumn = []
  for (const [key, value] of Object.entries(initialMemoryUsage)) {
    const diff = finalMemoryUsage[key] - value
    const prettyDiff = prettyBytes(diff, { signed: true })
    if (diff > 0) {
      diffColumn.push(chalk.red(prettyDiff))
    } else if (diff < 0) {
      diffColumn.push(chalk.green(prettyDiff))
    } else {
      diffColumn.push(chalk.grey(prettyDiff))
    }
  }
  tableData.push(diffColumn)
  console.info(table(tableData))
}

async function detect(job) {
  for (let i = 0; i <= 1000; i++) {
    await job()
    if (i % 1000 === 0) {
      displayMemoryUsageFromNode(initial)
    }

    if (process.memoryUsage().rss - initial.rss >= 1024 * 1024 * 100) {
      throw new Error('Memory limit reached')
    }
  }
}

async function memoryLeakDetect() {
  console.info(chalk.green('Zip file...'))
  await detect(async () => zip(path.resolve(__dirname, 'node_modules')))
  console.info(chalk.green('Unzip file...'))
  await detect(async () => unzip(path.resolve(__dirname, 'node_modules.zip')))
}

memoryLeakDetect().catch((err) => chalk.red(err))
