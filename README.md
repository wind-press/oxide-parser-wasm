# Oxide Parser WASM

This package allows you to run the [Tailwind CSS's Oxide](https://www.npmjs.com/package/@tailwindcss/oxide) parser from the browser and Node.js environments.

> [!NOTE]
>
> This package is automatically generated from the https://github.com/wind-press/oxide-parser-wasm repository.

## Features

- 🚀 **Browser & Node.js Compatible** - Works seamlessly in both environments
- 📦 **CDN Ready** - Use directly from esm.sh, jsdelivr, or unpkg without build tools
- ⚡ **WebAssembly Powered** - High-performance Tailwind CSS class extraction
- 🎯 **Zero Dependencies** - Lightweight and self-contained

## Installation

### NPM/Yarn/PNPM

Install the package via npm:

```bash
npm i @windpress/oxide-parser-wasm
```

### CDN (No Installation Required)

Use directly in the browser via CDN:

```html
<script type="module">
  import init, { find_tw_candidates } from 'https://esm.sh/@windpress/oxide-parser-wasm';
  
  // Initialize WASM (required - this loads the .wasm file)
  await init();
  
  // Now you can use the parser
  const candidates = find_tw_candidates('<div class="flex p-4">Hello</div>');
  console.log(candidates);
  // Output: ['flex', 'p-4', 'div', 'class', ...]
</script>
```

> **Note:** Unlike some WASM libraries, this package requires you to call `init()` before using `find_tw_candidates()`. This is standard for wasm-pack generated packages.

#### Available CDNs:

- **esm.sh**: `https://esm.sh/@windpress/oxide-parser-wasm`
- **jsdelivr**: `https://cdn.jsdelivr.net/npm/@windpress/oxide-parser-wasm`
- **unpkg**: `https://unpkg.com/@windpress/oxide-parser-wasm`

## Usage

### Node.js / Build Tools

```js
import init, { find_tw_candidates } from '@windpress/oxide-parser-wasm';

// Initialize WASM (required on first use)
await init();

// Extract Tailwind candidates
const candidates = find_tw_candidates(input);
```

### Browser (CDN)

See the [example-cdn.html](./example-cdn.html) file for a complete working example.

```html
<!DOCTYPE html>
<html>
<body>
  <script type="module">
    // Import from CDN
    import init, { find_tw_candidates } from 'https://esm.sh/@windpress/oxide-parser-wasm';
    
    // Initialize WASM
    await init();
    
    // Extract classes
    const html = '<div class="flex p-4 bg-blue-500">Hello World</div>';
    const candidates = find_tw_candidates(html);
    
    console.log(candidates);
    // Output: ['flex', 'p-4', 'bg-blue-500', 'div', 'class', ...]
  </script>
</body>
</html>
```

## Examples

### Basic Example

```js
import init, { find_tw_candidates } from '@windpress/oxide-parser-wasm';

// Initialize WASM module
await init();

const input = /** html */`
  <div class="relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12">
    <img src="/img/beams.jpg" alt="" class="absolute top-1/2 left-1/2 max-w-none -translate-x-1/2 -translate-y-1/2" width="1308">
    <div class="absolute inset-0 bg-[url(/img/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]"></div>
  </div>
`;

const candidates = find_tw_candidates(input);

console.log(candidates);
```

**Output:**

```js
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

### React/Vue/Svelte Components

```js
import init, { find_tw_candidates } from '@windpress/oxide-parser-wasm';

await init();

const component = `
  function Button({ children }) {
    return (
      <button className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600">
        {children}
      </button>
    );
  }
`;

const candidates = find_tw_candidates(component);
console.log(candidates);
// Output: ['px-4', 'py-2', 'bg-blue-500', 'text-white', 'rounded', 'hover:bg-blue-600', ...]
```

### Filter for Valid Tailwind Classes

The parser returns all potential candidates. You may want to filter them:

```js
import init, { find_tw_candidates } from '@windpress/oxide-parser-wasm';

await init();

const html = '<div class="flex p-4 custom-class">Content</div>';
const candidates = find_tw_candidates(html);

// Get unique values and filter out non-Tailwind patterns
const tailwindClasses = [...new Set(candidates)]
  .filter(c => 
    // Filter out HTML tags, attributes, etc.
    !['div', 'class', 'src', 'alt', 'href'].includes(c) &&
    // Keep Tailwind-like patterns
    /^[\w-:[\]()/.#]+$/.test(c)
  );

console.log(tailwindClasses);
// Output: ['flex', 'p-4', 'custom-class']
```

## API Reference

### `init()`

Initialize the WebAssembly module. Must be called before using `find_tw_candidates`.

```js
await init();
```

**Returns:** `Promise<void>`

### `find_tw_candidates(input: string): string[]`

Extract Tailwind CSS candidate classes from HTML/JSX/template strings.

**Parameters:**
- `input` (string): HTML, JSX, or any template string containing class names

**Returns:** `string[]` - Array of candidate class names (may include duplicates and false positives)

**Example:**
```js
const candidates = find_tw_candidates('<div class="flex p-4">Hello</div>');
// Returns: ['flex', 'p-4', 'div', 'class', ...]
```

## Live Demo

Open the included example files in your browser:
- **[index.html](./index.html)** - Compare local build vs CDN usage

## Building from Source

```bash
# Install Rust and wasm-pack
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install wasm-pack

# Build the project
npm run build

# This will build the WASM module using wasm-pack
# The output is already CDN-ready - no bundling needed!
```
