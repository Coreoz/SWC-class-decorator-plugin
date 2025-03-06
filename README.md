SWC-class-decorator-plugin
==========================

## Description

This is a plugin for the [vite-react-swc](https://github.com/vitejs/vite-plugin-react-swc) plugin compiler that
adds [plume-ts-di](https://github.com/Coreoz/plume-ts-di) needed information for dependency injection.

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

That's it, classes will be transformed to add the needed information for dependency injection.

You can add config options for logging and debugging.

```typescript
plugins: [['swc-class-decorator-plugin', { log: "Info" | "Debug" }]]
```

Build plugin
------------
`yarn build`

Run tests
---------
`yarn test`
