# Oxide Parser WASM

This package allow to run the [Tailwind CSS's Oxide](https://www.npmjs.com/package/@tailwindcss/oxide) parser from the browser.

> [!NOTE]
>
> This package is automatically generated from the https://github.com/wind-press/oxide-parser-wasm repository.

## Installation

Install the package via npm:

```bash
npm i @windpress/oxide-parser-wasm
```

## Usage

```js
import { find_tw_candidates } from '@windpress/oxide-parser-wasm';

const candidates = find_tw_candidates(input);
```

### Example 

```js
import { find_tw_candidates } from '@windpress/oxide-parser-wasm';

const input = /** html */`
    <div class="relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12"><img src="/img/beams.jpg" alt="" class="absolute top-1/2 left-1/2 max-w-none -translate-x-1/2 -translate-y-1/2" width="1308">
        <div class="absolute inset-0 bg-[url(/img/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]"></div>
    </div>
`;

const candidates = find_tw_candidates(input);

console.log(candidates);
```

Output:

```
[
  "justify-center",
  "jpg",
  "[mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]",
  "flex",
  "width",
  "-translate-y-1/2",
  "-translate-x-1/2",
  "sm:py-12",
  "class",
  "mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))",
  "img/beams",
  "inset-0",
  "bg-center",
  "top-1/2",
  "relative",
  "flex-col",
  "bg-[url(/img/grid.svg)]",
  "py-6",
  "left-1/2",
  "min-h-screen",
  "max-w-none",
  "img",
  "bg-gray-50",
  "src",
  "overflow-hidden",
  "absolute",
  "alt",
  "div"
]
```