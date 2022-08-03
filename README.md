# unzippers

Unzip library in NodeJS, powered by [napi-rs](https://napi.rs) and [zip-rs](https://github.com/zip-rs/zip).

## Install this package

```
npm install unzippers
```

## API

```ts
export interface Options {
  target?: string
}
export function unzip(
  entryPath: string,
  options?: Options | undefined | null,
  signal?: AbortSignal | undefined | null,
): Promise<void>
```

## Performance

### Hardware

```
MacBook Pro (13-inch, M1, 2020)
Chip Apple M1
Memory 8Gb
```

### Result

#### Unzip

```
Running "Unzip with file 5.076990127563477 MB" suite...
Progress: 100%
  extract-zip with file 5.076990127563477 MB:
    0.4 ops/s, ±1.35%   | slowest, 85.71% slower
  node-stream-zip with file 5.076990127563477 MB:
    1.1 ops/s, ±2.88%   | 60.71% slower
  unzippers with file 5.076990127563477 MB:
    2.8 ops/s, ±0.63%   | fastest
  adm-zip with file 5.076990127563477 MB:
    1.2 ops/s, ±3.12%   | 57.14% slower
Finished 4 cases!
  Fastest: unzippers with file 5.076990127563477 MB
  Slowest: extract-zip with file 5.076990127563477 MB
```
