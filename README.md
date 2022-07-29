# Zippers

Zip library in NodeJS, powered by [napi-rs](https://napi.rs) and [zip-rs](https://github.com/zip-rs/zip).

## Install this package

```
npm install zippers
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
export function zip(
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
  zippers with file 5.076990127563477 MB:
    2.8 ops/s, ±0.63%   | fastest
  adm-zip with file 5.076990127563477 MB:
    1.2 ops/s, ±3.12%   | 57.14% slower
Finished 4 cases!
  Fastest: zippers with file 5.076990127563477 MB
  Slowest: extract-zip with file 5.076990127563477 MB
```

#### Zip

```
Running "Zip with file 0.009429931640625 MB" suite...
Progress: 100%

  zippers with file 0.009429931640625 MB:
    8 792 ops/s, ±2.85%   | fastest

  adm-zip with file 0.009429931640625 MB:
    4 585 ops/s, ±6.55%   | slowest, 47.85% slower

Finished 2 cases!
  Fastest: zippers with file 0.009429931640625 MB
  Slowest: adm-zip with file 0.009429931640625 MB
```
