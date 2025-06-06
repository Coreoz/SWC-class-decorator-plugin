SWC-class-decorator-plugin
==========================

## Description

This is a [SWC](https://github.com/swc-project/swc) WASM plugin for Vite with the help of [vite-react-swc](https://github.com/vitejs/vite-plugin-react-swc) plugin
that adds [plume-ts-di](https://github.com/Coreoz/plume-ts-di) needed decorators for dependency injection.

Class files are transformed to add the class name and the dependencies for the dependency injection to work.
With this, you don't need to add the typescript transformers to your project.

This plugin is fully compatible with [plume-ts-di](https://github.com/Coreoz/plume-ts-di) and totally invalidates the
need of Typescript transformers.

## Example:

```typescript
export default class SampleService {
  constructor(private readonly sampleApi: SampleApi) {
  }

  sayHello(name: string) {
    return this.sampleApi.sample(name);
  }
}

```

Will be transformed to:

```typescript
export default class SampleService {
  constructor(private readonly sampleApi: SampleApi) {
  }

  sayHello(name: string) {
    return this.sampleApi.sample(name);
  }

  static get [Symbol.for("___CTOR_ARGS___")]() {
    return [
      "SampleApi"
    ];
  }

  static get [Symbol.for("___CTOR_NAME___")]() {
    return "SampleService";
  }
}
```

## Installation

Minimum requirements: `@vitejs/plugin-react-swc@3.8.0`

`yarn add -D swc-class-decorator-plugin`

Then in the `vite.config.ts` file add the plugin to the `plugin-react-swc` plugin.

```typescript
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react-swc';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [
    react({
      plugins: [['swc-class-decorator-plugin', {}]],
      useAtYourOwnRisk_mutateSwcOptions: (options) => {
        options.jsc!.experimental!.runPluginFirst = true;
      }
    }),
  ],
})
```

## Usage

You can add config options for logging and debugging.

```typescript
plugins: [['swc-class-decorator-plugin', { log: "Info" | "Debug" }]]
```

That's it, your classes will be transformed to add the needed information for dependency injection.

Development configuration
-------------------------
To work on this plugin, a sample project must be used.
Referencing the plugin in the sample project can be done with Yarn in the sample project using:
1. `yarn link /local/path/to/swc-class-decorator-plugin`
2. `yarn add -D swc-class-decorator-plugin@*`

Build plugin
------------
`yarn build`

Run tests
---------
`yarn test`

Structure
---------
Entry point: `src/lib.rs`

It uses the `plugin_transform` from `swc_core` to transform the class, and then redirect to `transform/src/lib.rs` to
process the file.
