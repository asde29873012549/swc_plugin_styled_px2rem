# swc_plugin_styled_px2rem

## Important Note
**Do not use this code in production.** This project is intended for experimental and educational purposes only. It may contain bugs and is not optimized for performance or security.

## Description
This project is a swc plugin to transform px values in styled components to rem at transpile time, in substitution for [babel-plugin-styled-components-px2rem](https://www.npmjs.com/package/babel-plugin-styled-components-px2rem).

## Features
- Converts px values to rem at transpile time
- Supports styled-components template literals
- Handles dynamic expressions by inserting a runtime px2rem function
- Configurable conversion options

## Installation

### Step 1: Install Rust
Make sure you have Rust installed on your machine. You can download and install Rust from the official website: [rust-lang.org](https://www.rust-lang.org/tools/install).

### Step 2: Clone the Repository
Clone this repository to your local machine:

```bash
git clone https://github.com/asde29873012549/swc_plugin_styled_px2rem.git
cd swc_plugin_styled_px2rem
```

### Step 3: Build the Plugin
Build the Rust project to generate the plugin:

You can either do

```bash
cargo build-wasi --release
```

or 

```bash
npm run prepublishOnly
```

### Step 4: Link the Plugin
Link the plugin to your local npm environment:

```bash
npm link
```

### Step 5: Link the Plugin to Your NextJS Project

```bash
npm link swc_plugin_styled_px2rem
```

### Step 6: Add Plugin Setting to `next.config.js`
In your Next.js project, add the plugin setting to your `next.config.js` file:

```js
module.exports = {
  experimental: {
    swcPlugins: [
      ['swc_plugin_styled_px2rem'],
    ],
  },
};
```


## Configuration Options

```javascript
{
  // Base font size for conversion
  "root_value": 3.75,
  
  // Decimal places in rem values  
  "unit_precision": 3,
  
  // Minimum px value to transform
  "min_pixel_value": 0.0,
  
  // Multiplication factor for conversion
  "multiplier": 1,
  
  // Enable runtime transformation
  "transform_interpolation": true,

  // Enable transforming px values in jsx attributes
  "transform_jsx_attributes": false,
  
  // Enable media query conversion
  "media_query": false
}
```


## Example

```js
// next.config.js
module.exports = {
  experimental: {
    swcPlugins: [
      ['swc_plugin_styled_px2rem', {
        "transform_jsx_attributes": true,
        "unit_precision": 1,
        "root_value": 16,
      }],
    ],
  },
};


// Before
const Avatar = ({ className, margin }) => (
    <div className={className} style={{ width: "100px" }} $margin/>
)

export default styled(Avatar)`
  padding: 32px;
  margin: ${props => props.$margin}px;
`


// After
const Avatar = ({ className }) => (
    <div className={className} style={{ width: "27.7rem" }} />
)

export default styled(Avatar)`
  padding: 2rem;
  margin: ${props => px2rem(props.margin)};
`
```
