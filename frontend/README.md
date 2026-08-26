# Frontend (Vue3 + Vite + TS)

低代码可视化编辑器与租户/总后台/代理商控制台。

## 规划目录

```
frontend/
  apps/
    admin/          # 总后台
    agent/          # 代理商后台
    tenant/         # 租户工作台（含 Telegram PC 风格画布）
  packages/
    editor/         # 拖拽画布核心
    ui/             # 共享 UI 组件
    api/            # 后端 API SDK
```

技术栈：Vue 3 + Vite + TypeScript + Element Plus（或 Naive UI）+ VueDraggable。

后续 Phase 将初始化 Vite 工程与 Telegram 风格布局壳。
