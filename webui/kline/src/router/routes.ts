import { RouteRecordRaw } from 'vue-router'

declare module 'vue-router' {
  // eslint-disable-next-line @typescript-eslint/no-empty-interface
  interface RouteMetaImpl {
  }
}

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [
      {
        path: '',
        component: () => import('pages/Swap.vue'),
        meta: {
          NeedLogined: false
        }
      },
      {
        path: '/swap',
        component: () => import('pages/Swap.vue'),
        meta: {
          NeedLogined: false
        }
      },
      {
        path: '/Meme',
        component: () => import('pages/MemeList.vue'),
        meta: {
          NeedLogined: false
        }
      },
      {
        path: '/create/meme',
        component: () => import('pages/CreateMeme.vue'),
        meta: {
          NeedLogined: false
        }
      }
    ]
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/Error404.vue')
  }
]

export default routes
