import ReactReconciler, { type EventPriority } from 'react-reconciler';
import { DefaultEventPriority } from 'react-reconciler/constants';
import type { JSX } from './jsx/runtime';
import type { Window } from '..';

type Container = Window;
type Type = any;
type Props = any;
type Instance = any;
type TextInstance = any;
type SuspenseInstance = any;
type HydratableInstance = any;
type FormInstance = any;
type PublicInstance = any;
type HostContext = any;
type ChildSet = any;
type TimeoutHandle = any;
type NoTimeout = any;
type TransitionStatus = any;

let currentPriority: EventPriority = DefaultEventPriority;

const reconciler = ReactReconciler<
  Type,
  Props,
  Container,
  Instance,
  TextInstance,
  SuspenseInstance,
  HydratableInstance,
  FormInstance,
  PublicInstance,
  HostContext,
  ChildSet,
  TimeoutHandle,
  NoTimeout,
  TransitionStatus
>({
  supportsMutation: true,
  supportsPersistence: false,
  createInstance: (type, props, rootContainer) => {
    console.log('createInstance', type, props);
    return { id: -1, type }; // dummy UElement for now
  },
  createTextInstance: (text) => {
    console.log('createTextInstance', text);
    return { id: -1, type: 'text' }; // dummy
  },
  appendInitialChild: (parent, child) => {
    console.log('appendInitialChild', parent, child);
  },
  finalizeInitialChildren: () => false,
  appendChildToContainer: (container, child) => {
    console.log('appendChildToContainer', container, child);
  },
  clearContainer: (container: Container) => {
    console.log('clearContainer', container);
  },
  shouldSetTextContent: () => false,
  getRootHostContext: () => ({}),
  getChildHostContext: (parentHostContext) => parentHostContext,
  getPublicInstance: (instance) => instance,
  prepareForCommit: () => null,
  resetAfterCommit: () => {},
  preparePortalMount: () => {},
  scheduleTimeout: (fn, delay) => setTimeout(fn, delay),
  cancelTimeout: (id) => clearTimeout(id),
  noTimeout: undefined,
  isPrimaryRenderer: false,
  getInstanceFromNode: function (
    node: any,
  ): ReactReconciler.Fiber | null | undefined {
    return null;
  },
  beforeActiveInstanceBlur: function (): void {},
  afterActiveInstanceBlur: function (): void {},
  prepareScopeUpdate: function (scopeInstance: any, instance: any): void {},
  getInstanceFromScope: function (scopeInstance: any) {},
  detachDeletedInstance: function (node: any): void {},
  supportsHydration: false,
  NotPendingTransition: undefined,
  HostTransitionContext: {
    $$typeof: Symbol.for('react.context'),
    _currentValue: null,
    _currentValue2: null,
  } as any,
  setCurrentUpdatePriority: (newPriority) => {
    currentPriority = newPriority;
  },
  getCurrentUpdatePriority: () => currentPriority,
  resolveUpdatePriority: () => DefaultEventPriority,
  resetFormInstance: () => {},
  requestPostPaintCallback: () => {},
  shouldAttemptEagerTransition: () => false,
  trackSchedulerEvent: function (): void {},
  resolveEventType: () => null,
  resolveEventTimeStamp: () => Date.now(),
  maySuspendCommit: () => false,
  preloadInstance: () => false,
  startSuspendingCommit: () => false,
  suspendInstance: () => {},
  waitForCommitToBeReady: ():
    | ((
        initiateCommit: (...args: unknown[]) => unknown,
      ) => (...args: unknown[]) => unknown)
    | null => {
    return null;
  },
});

export function render(window: Window, element: JSX.Element) {
  const container = reconciler.createContainer(
    window,
    1,
    null,
    false,
    null,
    '',
    console.error,
    console.error,
    console.error,
    () => {},
  );

  reconciler.updateContainer(element, container, null, () => {});

  return {
    dispose: () => reconciler.updateContainer(null, container, null, () => {}),
  };
}
