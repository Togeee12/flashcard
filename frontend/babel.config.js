module.exports = {
  presets: [
    '@vue/cli-plugin-babel/preset', // Vue CLI preset
    '@babel/preset-typescript',     // TypeScript preset
    [
      '@babel/preset-env',
      {
        modules: false, // Use 'false' to target ES modules
      },
    ],
  ],
};
