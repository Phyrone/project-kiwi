import { c as create_ssr_component, a as subscribe, b as add_attribute, d as add_styles, e as merge_ssr_styles, f as escape, v as validate_component, m as missing_component, n as noop, g as each, h as compute_rest_props, o as onDestroy, i as spread, j as escape_attribute_value, k as escape_object } from "../../chunks/ssr.js";
import { c as cn, t as toastState, u as useEffect } from "../../chunks/Toaster.svelte_svelte_type_style_lang.js";
const TOAST_LIFETIME = 4e3;
const GAP$1 = 14;
const TIME_BEFORE_UNMOUNT = 200;
const Toast = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let isFront;
  let isVisible;
  let toastType;
  let toastClass;
  let toastDescriptionClass;
  let heightIndex;
  let coords;
  let toastsHeightBefore;
  let disabled;
  let isPromiseLoadingOrInfiniteDuration;
  let $$unsubscribe_effect = noop, $$subscribe_effect = () => ($$unsubscribe_effect(), $$unsubscribe_effect = subscribe(effect, ($$value) => $$value), effect);
  let $heights, $$unsubscribe_heights;
  let $toasts, $$unsubscribe_toasts;
  const defaultClasses = {
    toast: "",
    title: "",
    description: "",
    loader: "",
    closeButton: "",
    cancelButton: "",
    actionButton: "",
    action: "",
    warning: "",
    error: "",
    success: "",
    default: "",
    info: "",
    loading: ""
  };
  const { toasts, heights, removeHeight, addHeight, dismiss } = toastState;
  $$unsubscribe_toasts = subscribe(toasts, (value) => $toasts = value);
  $$unsubscribe_heights = subscribe(heights, (value) => $heights = value);
  let { toast } = $$props;
  let { index } = $$props;
  let { expanded } = $$props;
  let { invert } = $$props;
  let { position } = $$props;
  let { visibleToasts } = $$props;
  let { expandByDefault } = $$props;
  let { closeButton } = $$props;
  let { interacting } = $$props;
  let { cancelButtonStyle = "" } = $$props;
  let { actionButtonStyle = "" } = $$props;
  let { duration = 4e3 } = $$props;
  let { descriptionClass = "" } = $$props;
  let { classes = {} } = $$props;
  let { unstyled = false } = $$props;
  let mounted = false;
  let removed = false;
  let swiping = false;
  let swipeOut = false;
  let offsetBeforeRemove = 0;
  let initialHeight = 0;
  let toastRef;
  let offset = 0;
  let closeTimerStartTimeRef = 0;
  let lastCloseTimerStartTimeRef = 0;
  async function updateHeights() {
    {
      return;
    }
  }
  function deleteToast() {
    removed = true;
    offsetBeforeRemove = offset;
    removeHeight(toast.id);
    setTimeout(
      () => {
        dismiss(toast.id);
      },
      TIME_BEFORE_UNMOUNT
    );
  }
  let timeoutId;
  let remainingTime = toast.duration || duration || TOAST_LIFETIME;
  function pauseTimer() {
    if (lastCloseTimerStartTimeRef < closeTimerStartTimeRef) {
      const elapsedTime = (/* @__PURE__ */ new Date()).getTime() - closeTimerStartTimeRef;
      remainingTime = remainingTime - elapsedTime;
    }
    lastCloseTimerStartTimeRef = (/* @__PURE__ */ new Date()).getTime();
  }
  function startTimer() {
    closeTimerStartTimeRef = (/* @__PURE__ */ new Date()).getTime();
    timeoutId = setTimeout(
      () => {
        toast.onAutoClose?.(toast);
        deleteToast();
      },
      remainingTime
    );
  }
  let effect;
  if ($$props.toast === void 0 && $$bindings.toast && toast !== void 0)
    $$bindings.toast(toast);
  if ($$props.index === void 0 && $$bindings.index && index !== void 0)
    $$bindings.index(index);
  if ($$props.expanded === void 0 && $$bindings.expanded && expanded !== void 0)
    $$bindings.expanded(expanded);
  if ($$props.invert === void 0 && $$bindings.invert && invert !== void 0)
    $$bindings.invert(invert);
  if ($$props.position === void 0 && $$bindings.position && position !== void 0)
    $$bindings.position(position);
  if ($$props.visibleToasts === void 0 && $$bindings.visibleToasts && visibleToasts !== void 0)
    $$bindings.visibleToasts(visibleToasts);
  if ($$props.expandByDefault === void 0 && $$bindings.expandByDefault && expandByDefault !== void 0)
    $$bindings.expandByDefault(expandByDefault);
  if ($$props.closeButton === void 0 && $$bindings.closeButton && closeButton !== void 0)
    $$bindings.closeButton(closeButton);
  if ($$props.interacting === void 0 && $$bindings.interacting && interacting !== void 0)
    $$bindings.interacting(interacting);
  if ($$props.cancelButtonStyle === void 0 && $$bindings.cancelButtonStyle && cancelButtonStyle !== void 0)
    $$bindings.cancelButtonStyle(cancelButtonStyle);
  if ($$props.actionButtonStyle === void 0 && $$bindings.actionButtonStyle && actionButtonStyle !== void 0)
    $$bindings.actionButtonStyle(actionButtonStyle);
  if ($$props.duration === void 0 && $$bindings.duration && duration !== void 0)
    $$bindings.duration(duration);
  if ($$props.descriptionClass === void 0 && $$bindings.descriptionClass && descriptionClass !== void 0)
    $$bindings.descriptionClass(descriptionClass);
  if ($$props.classes === void 0 && $$bindings.classes && classes !== void 0)
    $$bindings.classes(classes);
  if ($$props.unstyled === void 0 && $$bindings.unstyled && unstyled !== void 0)
    $$bindings.unstyled(unstyled);
  classes = { ...defaultClasses, ...classes };
  isFront = index === 0;
  isVisible = index + 1 <= visibleToasts;
  toastType = toast.type ?? "default";
  toastClass = toast.class || "";
  toastDescriptionClass = toast.descriptionClass || "";
  heightIndex = $heights.findIndex((height) => height.toastId === toast.id) || 0;
  coords = position.split("-");
  toastsHeightBefore = $heights.reduce(
    (prev, curr, reducerIndex) => {
      if (reducerIndex >= heightIndex)
        return prev;
      return prev + curr.height;
    },
    0
  );
  invert = toast.invert || invert;
  disabled = toastType === "loading";
  {
    {
      offset = heightIndex * GAP$1 + toastsHeightBefore;
    }
  }
  {
    toast.title, toast.description, updateHeights();
  }
  {
    if (toast.updated) {
      clearTimeout(timeoutId);
      remainingTime = toast.duration || duration || TOAST_LIFETIME;
      startTimer();
    }
  }
  isPromiseLoadingOrInfiniteDuration = toast.promise && toastType === "loading" || toast.duration === Number.POSITIVE_INFINITY;
  $$subscribe_effect(effect = useEffect(() => {
    if (!isPromiseLoadingOrInfiniteDuration) {
      if (expanded || interacting) {
        pauseTimer();
      } else {
        startTimer();
      }
    }
    return () => clearTimeout(timeoutId);
  }));
  {
    if (toast.delete) {
      deleteToast();
    }
  }
  $$unsubscribe_effect();
  $$unsubscribe_heights();
  $$unsubscribe_toasts();
  return `<li${add_attribute("aria-live", toast.important ? "assertive" : "polite", 0)} aria-atomic="true" role="status"${add_attribute("tabindex", 0, 0)}${add_attribute("class", cn($$props.class, toastClass, classes?.toast, toast?.classes?.toast, classes?.[toastType], toast?.classes?.[toastType]), 0)} data-sonner-toast=""${add_attribute("data-styled", !(toast.component || toast?.unstyled || unstyled), 0)}${add_attribute("data-mounted", mounted, 0)}${add_attribute("data-promise", Boolean(toast.promise), 0)}${add_attribute("data-removed", removed, 0)}${add_attribute("data-visible", isVisible, 0)}${add_attribute("data-y-position", coords[0], 0)}${add_attribute("data-x-position", coords[1], 0)}${add_attribute("data-index", index, 0)}${add_attribute("data-front", isFront, 0)}${add_attribute("data-swiping", swiping, 0)}${add_attribute("data-type", toastType, 0)}${add_attribute("data-invert", invert, 0)}${add_attribute("data-swipe-out", swipeOut, 0)}${add_attribute("data-expanded", Boolean(expanded || expandByDefault && mounted), 0)}${add_styles(merge_ssr_styles(escape(`${$$props.style} ${toast.style}`, true), {
    "--index": index,
    "--toasts-before": index,
    "--z-index": $toasts.length - index,
    "--offset": `${removed ? offsetBeforeRemove : offset}px`,
    "--initial-height": expandByDefault ? "auto" : `${initialHeight}px`
  }))}${add_attribute("this", toastRef, 0)}>${closeButton && !toast.component ? `<button aria-label="Close toast"${add_attribute("data-disabled", disabled, 0)} data-close-button${add_attribute("class", cn(classes?.closeButton, toast?.classes?.closeButton), 0)}><svg xmlns="http://www.w3.org/2000/svg" width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"><line x1="18" y1="6" x2="6" y2="18"></line><line x1="6" y1="6" x2="18" y2="18"></line></svg></button>` : ``} ${toast.component ? `${validate_component(toast.component || missing_component, "svelte:component").$$render($$result, Object.assign({}, toast.componentProps), {}, {})}` : `${toastType !== "default" || toast.icon || toast.promise ? `<div data-icon="">${(toast.promise || toastType === "loading") && !toast.icon ? `${slots["loading-icon"] ? slots["loading-icon"]({}) : ``}` : ``} ${toast.icon ? `${validate_component(toast.icon || missing_component, "svelte:component").$$render($$result, {}, {}, {})}` : `${toastType === "success" ? `${slots["success-icon"] ? slots["success-icon"]({}) : ``}` : `${toastType === "error" ? `${slots["error-icon"] ? slots["error-icon"]({}) : ``}` : `${toastType === "warning" ? `${slots["warning-icon"] ? slots["warning-icon"]({}) : ``}` : `${toastType === "info" ? `${slots["info-icon"] ? slots["info-icon"]({}) : ``}` : ``}`}`}`}`}</div>` : ``} <div data-content="">${toast.title ? `<div data-title=""${add_attribute("class", cn(classes?.title, toast?.classes?.title), 0)}>${typeof toast.title !== "string" ? `${validate_component(toast.title || missing_component, "svelte:component").$$render($$result, Object.assign({}, toast.componentProps), {}, {})}` : `${escape(toast.title)}`}</div>` : ``} ${toast.description ? `<div data-description=""${add_attribute("class", cn(descriptionClass, toastDescriptionClass, classes?.description, toast.classes?.description), 0)}>${typeof toast.description !== "string" ? `${validate_component(toast.description || missing_component, "svelte:component").$$render($$result, Object.assign({}, toast.componentProps), {}, {})}` : `${escape(toast.description)}`}</div>` : ``}</div> ${toast.cancel ? `<button data-button data-cancel${add_attribute("style", cancelButtonStyle, 0)}${add_attribute("class", cn(classes?.cancelButton, toast?.classes?.cancelButton), 0)}>${escape(toast.cancel.label)}</button>` : ``} ${toast.action ? `<button data-button=""${add_attribute("style", actionButtonStyle, 0)}${add_attribute("class", cn(classes?.actionButton, toast?.classes?.actionButton), 0)}>${escape(toast.action.label)}</button>` : ``}`}</li>`;
});
const Loader = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { visible } = $$props;
  const bars = Array(12).fill(0);
  if ($$props.visible === void 0 && $$bindings.visible && visible !== void 0)
    $$bindings.visible(visible);
  return `<div class="sonner-loading-wrapper"${add_attribute("data-visible", visible, 0)}><div class="sonner-spinner">${each(bars, (_, i) => {
    return `<div class="sonner-loading-bar"></div>`;
  })}</div></div>`;
});
const Icon = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let { type = "success" } = $$props;
  if ($$props.type === void 0 && $$bindings.type && type !== void 0)
    $$bindings.type(type);
  return `${type === "success" ? `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" height="20" width="20"><path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.857-9.809a.75.75 0 00-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 10-1.06 1.061l2.5 2.5a.75.75 0 001.137-.089l4-5.5z" clip-rule="evenodd"></path></svg>` : `${type === "error" ? `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" height="20" width="20"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-8-5a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5A.75.75 0 0110 5zm0 10a1 1 0 100-2 1 1 0 000 2z" clip-rule="evenodd"></path></svg>` : `${type === "info" ? `<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 20 20" fill="currentColor" height="20" width="20"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a.75.75 0 000 1.5h.253a.25.25 0 01.244.304l-.459 2.066A1.75 1.75 0 0010.747 15H11a.75.75 0 000-1.5h-.253a.25.25 0 01-.244-.304l.459-2.066A1.75 1.75 0 009.253 9H9z" clip-rule="evenodd"></path></svg>` : `${type === "warning" ? `<svg viewBox="0 0 64 64" fill="currentColor" height="20" width="20" xmlns="http://www.w3.org/2000/svg"><path d="M32.427,7.987c2.183,0.124 4,1.165 5.096,3.281l17.936,36.208c1.739,3.66 -0.954,8.585 -5.373,8.656l-36.119,0c-4.022,-0.064 -7.322,-4.631 -5.352,-8.696l18.271,-36.207c0.342,-0.65 0.498,-0.838 0.793,-1.179c1.186,-1.375 2.483,-2.111 4.748,-2.063Zm-0.295,3.997c-0.687,0.034 -1.316,0.419 -1.659,1.017c-6.312,11.979 -12.397,24.081 -18.301,36.267c-0.546,1.225 0.391,2.797 1.762,2.863c12.06,0.195 24.125,0.195 36.185,0c1.325,-0.064 2.321,-1.584 1.769,-2.85c-5.793,-12.184 -11.765,-24.286 -17.966,-36.267c-0.366,-0.651 -0.903,-1.042 -1.79,-1.03Z"></path><path d="M33.631,40.581l-3.348,0l-0.368,-16.449l4.1,0l-0.384,16.449Zm-3.828,5.03c0,-0.609 0.197,-1.113 0.592,-1.514c0.396,-0.4 0.935,-0.601 1.618,-0.601c0.684,0 1.223,0.201 1.618,0.601c0.395,0.401 0.593,0.905 0.593,1.514c0,0.587 -0.193,1.078 -0.577,1.473c-0.385,0.395 -0.929,0.593 -1.634,0.593c-0.705,0 -1.249,-0.198 -1.634,-0.593c-0.384,-0.395 -0.576,-0.886 -0.576,-1.473Z"></path></svg>` : ``}`}`}`}`;
});
const css = {
  code: "html[dir='ltr'],[data-sonner-toaster][dir='ltr']{--toast-icon-margin-start:-3px;--toast-icon-margin-end:4px;--toast-svg-margin-start:-1px;--toast-svg-margin-end:0px;--toast-button-margin-start:auto;--toast-button-margin-end:0;--toast-close-button-start:0;--toast-close-button-end:unset;--toast-close-button-transform:translate(-35%, -35%)}html[dir='rtl'],[data-sonner-toaster][dir='rtl']{--toast-icon-margin-start:4px;--toast-icon-margin-end:-3px;--toast-svg-margin-start:0px;--toast-svg-margin-end:-1px;--toast-button-margin-start:0;--toast-button-margin-end:auto;--toast-close-button-start:unset;--toast-close-button-end:0;--toast-close-button-transform:translate(35%, -35%)}[data-sonner-toaster]{position:fixed;width:var(--width);font-family:ui-sans-serif, system-ui, -apple-system, BlinkMacSystemFont, Segoe UI, Roboto, Helvetica Neue, Arial,\n		Noto Sans, sans-serif, Apple Color Emoji, Segoe UI Emoji, Segoe UI Symbol, Noto Color Emoji;--gray1:hsl(0, 0%, 99%);--gray2:hsl(0, 0%, 97.3%);--gray3:hsl(0, 0%, 95.1%);--gray4:hsl(0, 0%, 93%);--gray5:hsl(0, 0%, 90.9%);--gray6:hsl(0, 0%, 88.7%);--gray7:hsl(0, 0%, 85.8%);--gray8:hsl(0, 0%, 78%);--gray9:hsl(0, 0%, 56.1%);--gray10:hsl(0, 0%, 52.3%);--gray11:hsl(0, 0%, 43.5%);--gray12:hsl(0, 0%, 9%);--border-radius:8px;box-sizing:border-box;padding:0;margin:0;list-style:none;outline:none;z-index:999999999}[data-sonner-toaster][data-x-position='right']{right:max(var(--offset), env(safe-area-inset-right))}[data-sonner-toaster][data-x-position='left']{left:max(var(--offset), env(safe-area-inset-left))}[data-sonner-toaster][data-x-position='center']{left:50%;transform:translateX(-50%)}[data-sonner-toaster][data-y-position='top']{top:max(var(--offset), env(safe-area-inset-top))}[data-sonner-toaster][data-y-position='bottom']{bottom:max(var(--offset), env(safe-area-inset-bottom))}[data-sonner-toast]{--y:translateY(100%);--lift-amount:calc(var(--lift) * var(--gap));z-index:var(--z-index);position:absolute;opacity:0;transform:var(--y);filter:blur(0);touch-action:none;transition:transform 400ms, opacity 400ms, height 400ms, box-shadow 200ms;box-sizing:border-box;outline:none;overflow-wrap:anywhere}[data-sonner-toast][data-styled='true']{padding:16px;background:var(--normal-bg);border:1px solid var(--normal-border);color:var(--normal-text);border-radius:var(--border-radius);box-shadow:0px 4px 12px rgba(0, 0, 0, 0.1);width:var(--width);font-size:13px;display:flex;align-items:center;gap:6px}[data-sonner-toast]:focus-visible{box-shadow:0px 4px 12px rgba(0, 0, 0, 0.1), 0 0 0 2px rgba(0, 0, 0, 0.2)}[data-sonner-toast][data-y-position='top']{top:0;--y:translateY(-100%);--lift:1;--lift-amount:calc(1 * var(--gap))}[data-sonner-toast][data-y-position='bottom']{bottom:0;--y:translateY(100%);--lift:-1;--lift-amount:calc(var(--lift) * var(--gap))}[data-sonner-toast] [data-description]{font-weight:400;line-height:1.4;color:inherit}[data-sonner-toast] [data-title]{font-weight:500;line-height:1.5;color:inherit}[data-sonner-toast] [data-icon]{display:flex;height:16px;width:16px;position:relative;justify-content:flex-start;align-items:center;flex-shrink:0;margin-left:var(--toast-icon-margin-start);margin-right:var(--toast-icon-margin-end)}[data-sonner-toast][data-promise='true'] [data-icon]>svg{opacity:0;transform:scale(0.8);transform-origin:center;animation:sonner-fade-in 300ms ease forwards}[data-sonner-toast] [data-icon]>*{flex-shrink:0}[data-sonner-toast] [data-icon] svg{margin-left:var(--toast-svg-margin-start);margin-right:var(--toast-svg-margin-end)}[data-sonner-toast] [data-content]{display:flex;flex-direction:column;gap:2px}[data-sonner-toast] [data-button]{border-radius:4px;padding-left:8px;padding-right:8px;height:24px;font-size:12px;color:var(--normal-bg);background:var(--normal-text);margin-left:var(--toast-button-margin-start);margin-right:var(--toast-button-margin-end);border:none;cursor:pointer;outline:none;display:flex;align-items:center;flex-shrink:0;transition:opacity 400ms, box-shadow 200ms}[data-sonner-toast] [data-button]:focus-visible{box-shadow:0 0 0 2px rgba(0, 0, 0, 0.4)}[data-sonner-toast] [data-button]:first-of-type{margin-left:var(--toast-button-margin-start);margin-right:var(--toast-button-margin-end)}[data-sonner-toast] [data-cancel]{color:var(--normal-text);background:rgba(0, 0, 0, 0.08)}[data-sonner-toast][data-theme='dark'] [data-cancel]{background:rgba(255, 255, 255, 0.3)}[data-sonner-toast] [data-close-button]{position:absolute;left:var(--toast-close-button-start);right:var(--toast-close-button-end);top:0;height:20px;width:20px;display:flex;justify-content:center;align-items:center;padding:0;background:var(--gray1);color:var(--gray12);border:1px solid var(--gray4);transform:var(--toast-close-button-transform);border-radius:50%;cursor:pointer;z-index:1;transition:opacity 100ms, background 200ms, border-color 200ms}[data-sonner-toast] [data-close-button]:focus-visible{box-shadow:0px 4px 12px rgba(0, 0, 0, 0.1), 0 0 0 2px rgba(0, 0, 0, 0.2)}[data-sonner-toast] [data-disabled='true']{cursor:not-allowed}[data-sonner-toast]:hover [data-close-button]:hover{background:var(--gray2);border-color:var(--gray5)}[data-sonner-toast][data-swiping='true']:before{content:'';position:absolute;left:0;right:0;height:100%;z-index:-1}[data-sonner-toast][data-y-position='top'][data-swiping='true']:before{bottom:50%;transform:scaleY(3) translateY(50%)}[data-sonner-toast][data-y-position='bottom'][data-swiping='true']:before{top:50%;transform:scaleY(3) translateY(-50%)}[data-sonner-toast][data-swiping='false'][data-removed='true']:before{content:'';position:absolute;inset:0;transform:scaleY(2)}[data-sonner-toast]:after{content:'';position:absolute;left:0;height:calc(var(--gap) + 1px);bottom:100%;width:100%}[data-sonner-toast][data-mounted='true']{--y:translateY(0);opacity:1}[data-sonner-toast][data-expanded='false'][data-front='false']{--scale:var(--toasts-before) * 0.05 + 1;--y:translateY(calc(var(--lift-amount) * var(--toasts-before))) scale(calc(-1 * var(--scale)));height:var(--front-toast-height)}[data-sonner-toast]>*{transition:opacity 400ms}[data-sonner-toast][data-expanded='false'][data-front='false'][data-styled='true']>*{opacity:0}[data-sonner-toast][data-visible='false']{opacity:0;pointer-events:none}[data-sonner-toast][data-mounted='true'][data-expanded='true']{--y:translateY(calc(var(--lift) * var(--offset)));height:var(--initial-height)}[data-sonner-toast][data-removed='true'][data-front='true'][data-swipe-out='false']{--y:translateY(calc(var(--lift) * -100%));opacity:0}[data-sonner-toast][data-removed='true'][data-front='false'][data-swipe-out='false'][data-expanded='true']{--y:translateY(calc(var(--lift) * var(--offset) + var(--lift) * -100%));opacity:0}[data-sonner-toast][data-removed='true'][data-front='false'][data-swipe-out='false'][data-expanded='false']{--y:translateY(40%);opacity:0;transition:transform 500ms, opacity 200ms}[data-sonner-toast][data-removed='true'][data-front='false']:before{height:calc(var(--initial-height) + 20%)}[data-sonner-toast][data-swiping='true']{transform:var(--y) translateY(var(--swipe-amount, 0px));transition:none}[data-sonner-toast][data-swipe-out='true'][data-y-position='bottom'],[data-sonner-toast][data-swipe-out='true'][data-y-position='top']{animation:swipe-out 200ms ease-out forwards}@keyframes swipe-out{from{transform:translateY(calc(var(--lift) * var(--offset) + var(--swipe-amount)));opacity:1}to{transform:translateY(calc(var(--lift) * var(--offset) + var(--swipe-amount) + var(--lift) * -100%));opacity:0}}@media(max-width: 600px){[data-sonner-toaster]{position:fixed;--mobile-offset:16px;right:var(--mobile-offset);left:var(--mobile-offset);width:100%}[data-sonner-toaster] [data-sonner-toast]{left:0;right:0;width:calc(100% - 32px)}[data-sonner-toaster][data-x-position='left']{left:var(--mobile-offset)}[data-sonner-toaster][data-y-position='bottom']{bottom:20px}[data-sonner-toaster][data-y-position='top']{top:20px}[data-sonner-toaster][data-x-position='center']{left:var(--mobile-offset);right:var(--mobile-offset);transform:none}}[data-sonner-toaster][data-theme='light']{--normal-bg:#fff;--normal-border:var(--gray4);--normal-text:var(--gray12);--success-bg:hsl(143, 85%, 96%);--success-border:hsl(145, 92%, 91%);--success-text:hsl(140, 100%, 27%);--info-bg:hsl(208, 100%, 97%);--info-border:hsl(221, 91%, 91%);--info-text:hsl(210, 92%, 45%);--warning-bg:hsl(49, 100%, 97%);--warning-border:hsl(49, 91%, 91%);--warning-text:hsl(31, 92%, 45%);--error-bg:hsl(359, 100%, 97%);--error-border:hsl(359, 100%, 94%);--error-text:hsl(360, 100%, 45%)}[data-sonner-toaster][data-theme='light'] [data-sonner-toast][data-invert='true']{--normal-bg:#000;--normal-border:hsl(0, 0%, 20%);--normal-text:var(--gray1)}[data-sonner-toaster][data-theme='dark'] [data-sonner-toast][data-invert='true']{--normal-bg:#fff;--normal-border:var(--gray3);--normal-text:var(--gray12)}[data-sonner-toaster][data-theme='dark']{--normal-bg:#000;--normal-border:hsl(0, 0%, 20%);--normal-text:var(--gray1);--success-bg:hsl(150, 100%, 6%);--success-border:hsl(147, 100%, 12%);--success-text:hsl(150, 86%, 65%);--info-bg:hsl(215, 100%, 6%);--info-border:hsl(223, 100%, 12%);--info-text:hsl(216, 87%, 65%);--warning-bg:hsl(64, 100%, 6%);--warning-border:hsl(60, 100%, 12%);--warning-text:hsl(46, 87%, 65%);--error-bg:hsl(358, 76%, 10%);--error-border:hsl(357, 89%, 16%);--error-text:hsl(358, 100%, 81%)}[data-rich-colors='true'] [data-sonner-toast][data-type='success']{background:var(--success-bg);border-color:var(--success-border);color:var(--success-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='success'] [data-close-button]{background:var(--success-bg);border-color:var(--success-border);color:var(--success-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='info']{background:var(--info-bg);border-color:var(--info-border);color:var(--info-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='info'] [data-close-button]{background:var(--info-bg);border-color:var(--info-border);color:var(--info-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='warning']{background:var(--warning-bg);border-color:var(--warning-border);color:var(--warning-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='warning'] [data-close-button]{background:var(--warning-bg);border-color:var(--warning-border);color:var(--warning-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='error']{background:var(--error-bg);border-color:var(--error-border);color:var(--error-text)}[data-rich-colors='true'] [data-sonner-toast][data-type='error'] [data-close-button]{background:var(--error-bg);border-color:var(--error-border);color:var(--error-text)}.sonner-loading-wrapper{--size:16px;height:var(--size);width:var(--size);position:absolute;inset:0;z-index:10}.sonner-loading-wrapper[data-visible='false']{transform-origin:center;animation:sonner-fade-out 0.2s ease forwards}.sonner-spinner{position:relative;top:50%;left:50%;height:var(--size);width:var(--size)}.sonner-loading-bar{animation:sonner-spin 1.2s linear infinite;background:var(--gray11);border-radius:6px;height:8%;left:-10%;position:absolute;top:-3.9%;width:24%}.sonner-loading-bar:nth-child(1){animation-delay:-1.2s;transform:rotate(0.0001deg) translate(146%)}.sonner-loading-bar:nth-child(2){animation-delay:-1.1s;transform:rotate(30deg) translate(146%)}.sonner-loading-bar:nth-child(3){animation-delay:-1s;transform:rotate(60deg) translate(146%)}.sonner-loading-bar:nth-child(4){animation-delay:-0.9s;transform:rotate(90deg) translate(146%)}.sonner-loading-bar:nth-child(5){animation-delay:-0.8s;transform:rotate(120deg) translate(146%)}.sonner-loading-bar:nth-child(6){animation-delay:-0.7s;transform:rotate(150deg) translate(146%)}.sonner-loading-bar:nth-child(7){animation-delay:-0.6s;transform:rotate(180deg) translate(146%)}.sonner-loading-bar:nth-child(8){animation-delay:-0.5s;transform:rotate(210deg) translate(146%)}.sonner-loading-bar:nth-child(9){animation-delay:-0.4s;transform:rotate(240deg) translate(146%)}.sonner-loading-bar:nth-child(10){animation-delay:-0.3s;transform:rotate(270deg) translate(146%)}.sonner-loading-bar:nth-child(11){animation-delay:-0.2s;transform:rotate(300deg) translate(146%)}.sonner-loading-bar:nth-child(12){animation-delay:-0.1s;transform:rotate(330deg) translate(146%)}@keyframes sonner-fade-in{0%{opacity:0;transform:scale(0.8)}100%{opacity:1;transform:scale(1)}}@keyframes sonner-fade-out{0%{opacity:1;transform:scale(1)}100%{opacity:0;transform:scale(0.8)}}@keyframes sonner-spin{0%{opacity:1}100%{opacity:0.15}}@media(prefers-reduced-motion){[data-sonner-toast],[data-sonner-toast]>*,.sonner-loading-bar{transition:none !important;animation:none !important}}.sonner-loader{position:absolute;top:50%;left:50%;transform:translate(-50%, -50%);transform-origin:center;transition:opacity 200ms, transform 200ms}.sonner-loader[data-visible='false']{opacity:0;transform:scale(0.8) translate(-50%, -50%)}",
  map: null
};
const VISIBLE_TOASTS_AMOUNT = 3;
const VIEWPORT_OFFSET = "32px";
const TOAST_WIDTH = 356;
const GAP = 14;
function getInitialTheme(t) {
  if (t !== "system") {
    return t;
  }
  if (typeof window !== "undefined") {
    if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
      return "dark";
    }
    return "light";
  }
  return "light";
}
function getDocumentDirection() {
  if (typeof window === "undefined")
    return "ltr";
  if (typeof document === "undefined")
    return "ltr";
  const dirAttribute = document.documentElement.getAttribute("dir");
  if (dirAttribute === "auto" || !dirAttribute) {
    return window.getComputedStyle(document.documentElement).direction;
  }
  return dirAttribute;
}
const Toaster = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  let possiblePositions;
  let hotkeyLabel;
  let $$restProps = compute_rest_props($$props, [
    "invert",
    "theme",
    "position",
    "hotkey",
    "richColors",
    "expand",
    "duration",
    "visibleToasts",
    "closeButton",
    "toastOptions",
    "offset",
    "dir"
  ]);
  let $toasts, $$unsubscribe_toasts;
  let $heights, $$unsubscribe_heights;
  let { invert = false } = $$props;
  let { theme = "light" } = $$props;
  let { position = "bottom-right" } = $$props;
  let { hotkey = ["altKey", "KeyT"] } = $$props;
  let { richColors = false } = $$props;
  let { expand = false } = $$props;
  let { duration = 4e3 } = $$props;
  let { visibleToasts = VISIBLE_TOASTS_AMOUNT } = $$props;
  let { closeButton = false } = $$props;
  let { toastOptions = {} } = $$props;
  let { offset = null } = $$props;
  let { dir = getDocumentDirection() } = $$props;
  const { toasts, heights, reset } = toastState;
  $$unsubscribe_toasts = subscribe(toasts, (value) => $toasts = value);
  $$unsubscribe_heights = subscribe(heights, (value) => $heights = value);
  let expanded = false;
  let interacting = false;
  let actualTheme = getInitialTheme(theme);
  let listRef;
  onDestroy(() => {
  });
  if ($$props.invert === void 0 && $$bindings.invert && invert !== void 0)
    $$bindings.invert(invert);
  if ($$props.theme === void 0 && $$bindings.theme && theme !== void 0)
    $$bindings.theme(theme);
  if ($$props.position === void 0 && $$bindings.position && position !== void 0)
    $$bindings.position(position);
  if ($$props.hotkey === void 0 && $$bindings.hotkey && hotkey !== void 0)
    $$bindings.hotkey(hotkey);
  if ($$props.richColors === void 0 && $$bindings.richColors && richColors !== void 0)
    $$bindings.richColors(richColors);
  if ($$props.expand === void 0 && $$bindings.expand && expand !== void 0)
    $$bindings.expand(expand);
  if ($$props.duration === void 0 && $$bindings.duration && duration !== void 0)
    $$bindings.duration(duration);
  if ($$props.visibleToasts === void 0 && $$bindings.visibleToasts && visibleToasts !== void 0)
    $$bindings.visibleToasts(visibleToasts);
  if ($$props.closeButton === void 0 && $$bindings.closeButton && closeButton !== void 0)
    $$bindings.closeButton(closeButton);
  if ($$props.toastOptions === void 0 && $$bindings.toastOptions && toastOptions !== void 0)
    $$bindings.toastOptions(toastOptions);
  if ($$props.offset === void 0 && $$bindings.offset && offset !== void 0)
    $$bindings.offset(offset);
  if ($$props.dir === void 0 && $$bindings.dir && dir !== void 0)
    $$bindings.dir(dir);
  $$result.css.add(css);
  possiblePositions = Array.from(new Set([
    position,
    ...$toasts.filter((toast) => toast.position).map((toast) => toast.position)
  ].filter(Boolean)));
  hotkeyLabel = hotkey.join("+").replace(/Key/g, "").replace(/Digit/g, "");
  {
    if ($toasts.length <= 1) {
      expanded = false;
    }
  }
  {
    {
      if (theme !== "system") {
        actualTheme = theme;
      }
      if (typeof window !== "undefined") {
        if (theme === "system") {
          if (window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches) {
            actualTheme = "dark";
          } else {
            actualTheme = "light";
          }
        }
        window.matchMedia("(prefers-color-scheme: dark)").addEventListener("change", ({ matches }) => {
          actualTheme = matches ? "dark" : "light";
        });
      }
    }
  }
  $$unsubscribe_toasts();
  $$unsubscribe_heights();
  return `${$toasts.length > 0 ? `<section${add_attribute("aria-label", `Notifications ${hotkeyLabel}`, 0)}${add_attribute("tabindex", -1, 0)}>${each(possiblePositions, (position2, index) => {
    return `<ol${spread(
      [
        { tabindex: escape_attribute_value(-1) },
        {
          class: escape_attribute_value($$props.class)
        },
        { "data-sonner-toaster": true },
        {
          "data-theme": escape_attribute_value(actualTheme)
        },
        {
          "data-rich-colors": escape_attribute_value(richColors)
        },
        {
          dir: escape_attribute_value(dir === "auto" ? getDocumentDirection() : dir)
        },
        {
          "data-y-position": escape_attribute_value(position2.split("-")[0])
        },
        {
          "data-x-position": escape_attribute_value(position2.split("-")[1])
        },
        {
          style: escape_attribute_value($$props.style)
        },
        escape_object($$restProps)
      ],
      {
        styles: {
          "--front-toast-height": `${$heights[0]?.height}px`,
          "--offset": typeof offset === "number" ? `${offset}px` : offset || VIEWPORT_OFFSET,
          "--width": `${TOAST_WIDTH}px`,
          "--gap": `${GAP}px`
        }
      }
    )}${add_attribute("this", listRef, 0)}>${each($toasts.filter((toast) => !toast.position && index === 0 || toast.position === position2), (toast, index2) => {
      return `${validate_component(Toast, "Toast").$$render(
        $$result,
        {
          index: index2,
          toast,
          invert,
          visibleToasts,
          closeButton,
          interacting,
          position: position2,
          expandByDefault: expand,
          expanded,
          actionButtonStyle: toastOptions?.actionButtonStyle || "",
          cancelButtonStyle: toastOptions?.cancelButtonStyle || "",
          class: toastOptions?.class || "",
          descriptionClass: toastOptions?.descriptionClass || "",
          classes: toastOptions.classes || {},
          duration: toastOptions?.duration ?? duration,
          unstyled: toastOptions.unstyled || false
        },
        {},
        {
          "info-icon": () => {
            return `${slots["info-icon"] ? slots["info-icon"]({ slot: "info-icon" }) : ` ${validate_component(Icon, "Icon").$$render($$result, { type: "info" }, {}, {})} `}`;
          },
          "warning-icon": () => {
            return `${slots["warning-icon"] ? slots["warning-icon"]({ slot: "warning-icon" }) : ` ${validate_component(Icon, "Icon").$$render($$result, { type: "warning" }, {}, {})} `}`;
          },
          "error-icon": () => {
            return `${slots["error-icon"] ? slots["error-icon"]({ slot: "error-icon" }) : ` ${validate_component(Icon, "Icon").$$render($$result, { type: "error" }, {}, {})} `}`;
          },
          "success-icon": () => {
            return `${slots["success-icon"] ? slots["success-icon"]({ slot: "success-icon" }) : ` ${validate_component(Icon, "Icon").$$render($$result, { type: "success" }, {}, {})} `}`;
          },
          "loading-icon": () => {
            return `${slots["loading-icon"] ? slots["loading-icon"]({ slot: "loading-icon" }) : ` ${validate_component(Loader, "Loader").$$render($$result, { visible: toast.type === "loading" }, {}, {})} `}`;
          }
        }
      )}`;
    })} </ol>`;
  })}</section>` : ``}`;
});
const Layout = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `${validate_component(Toaster, "Toaster").$$render($$result, { richColors: true }, {}, {})} ${slots.default ? slots.default({}) : ``}`;
});
export {
  Layout as default
};
