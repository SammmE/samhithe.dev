import * as runtime from 'react/jsx-runtime'

const useMDXComponent = (code: string) => {
  if (!code) return null
  const fn = new Function(code)
  return fn({ ...runtime }).default
}

export const MDXContent = ({ code, components }: { code: string, components?: any }) => {
  const Component = useMDXComponent(code)
  if (!Component) return null
  return <Component components={components} />
}
