# Oxide Parser PHP

This package provides a PHP interface to [Tailwind CSS's Oxide](https://www.npmjs.com/package/@tailwindcss/oxide) parser.


> [!NOTE]
>
> This package is automatically generated from the https://github.com/wind-press/oxide-parser-wasm repository.

## Supported OS

- **Linux** x86_64
- **Windows** x86_64
- **MacOS** aarch64

## PHP Requirements

This package require the [FFI extension](https://www.php.net/manual/en/intro.ffi.php) to be enabled.

## Installation

Install the package via composer:

```bash
composer require windpress/oxide-parser
```


## Available Methods

**find_tw_candidates** - Find Tailwind CSS class candidates in the given HTML content.

```php
namespace WindPress\OxideParser;

class Parser
{
    /**
     * Find Tailwind CSS class candidates in the given HTML content.
     *
     * @param string $css_content
     * @return array
     */
    public function find_tw_candidates(string $css_content): array
}
```

## Usage

```php
use WindPress\OxideParser\Parser;


$output = Parser::get_instance()->find_tw_candidates($input);
```


## Example

Example of how to use the package to find Tailwind CSS class candidates in a given HTML content.

```php
$input = <<<HTML
<div class="relative flex min-h-screen flex-col justify-center overflow-hidden bg-gray-50 py-6 sm:py-12"><img src="/img/beams.jpg" alt="" class="absolute top-1/2 left-1/2 max-w-none -translate-x-1/2 -translate-y-1/2" width="1308"><div class="absolute inset-0 bg-[url(/img/grid.svg)] bg-center [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]"></div></div>
HTML;

$output = \WindPress\OxideParser\Parser::get_instance()->find_tw_candidates($input);

print_r($output);
```

Output:

```
Array
(
    [0] => justify-center
    [1] => inset-0
    [2] => bg-center
    [3] => img
    [4] => jpg
    [5] => div
    [6] => -translate-x-1/2
    [7] => -translate-y-1/2
    [8] => max-w-none
    [9] => [mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))]
    [10] => sm:py-12
    [11] => width
    [12] => mask-image:linear-gradient(180deg,white,rgba(255,255,255,0))
    [13] => bg-[url(/img/grid.svg)]
    [14] => class
    [15] => min-h-screen
    [16] => left-1/2
    [17] => absolute
    [18] => top-1/2
    [19] => bg-gray-50
    [20] => src
    [21] => flex
    [22] => overflow-hidden
    [23] => py-6
    [24] => img/beams
    [25] => flex-col
    [26] => relative
    [27] => alt
)
```
