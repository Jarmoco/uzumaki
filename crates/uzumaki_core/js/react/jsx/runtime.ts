import type { ReactNode } from 'react';
import { createElement } from 'react';

export namespace JSX {
  export type Element = ReactNode;

  export interface ElementClass {}

  export interface IntrinsicElements {
    view: {
      x?: number;
      y?: number;
      w?: number | 'full';
      h?: number | 'full';
      'hover:bg'?: string;
      children?: any;
      flex?: boolean | 'col' | 'row';
      items?: 'start' | 'end' | 'center' | 'stretch' | 'baseline';
      justify?: 'start' | 'end' | 'center' | 'between' | 'around' | 'evenly';
      px?: number;
      py?: number;
      p?: number;
      pt?: number;
      pb?: number;
      gap?: number;
    };
    text: {
      children?: Element;
    };
    p: {
      children?: Element;
    };
    button: {
      children?: Element;
      onClick?: () => void;
    };
  }
}

export function jsx(
  type: string,
  props: Record<string, any>,
  key?: string,
): JSX.Element {
  return createElement(type, props, key);
}

export function jsxs(
  type: string,
  props: Record<string, any>,
  key?: string,
): JSX.Element {
  return createElement(type, props, key);
}

export const jsxDEV = jsx;

export const Fragment = Symbol('Fragment');
