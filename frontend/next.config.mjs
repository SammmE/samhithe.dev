import { build } from 'velite'

/** @type {import('next').NextConfig} */
const nextConfig = {
    reactStrictMode: true,
    pageExtensions: ['js', 'jsx', 'ts', 'tsx'],
    webpack: (config) => {
        config.plugins.push(new VeliteWebpackPlugin())
        return config
    }
};

class VeliteWebpackPlugin {
  static started = false
  apply(compiler) {
    // Only run this once
    compiler.hooks.beforeCompile.tapPromise('VeliteWebpackPlugin', async () => {
      if (VeliteWebpackPlugin.started) return
      VeliteWebpackPlugin.started = true
      const dev = compiler.options.mode === 'development'
      await build({ watch: dev, clean: !dev })
    })
  }
}

export default nextConfig;
