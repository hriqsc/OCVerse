import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      // localhost/ -> automatically redirects to localhost/hub
      path: '/',
      redirect: '/hub',
    },
    {
      path: '/hub',
      name: 'hub',
      component: () => import('@/views/HubView.vue'),
    },
    {
      path: '/hub/upload',
      name: 'oc-upload',
      component: () => import('@/views/UploadView.vue'),
    },
    {
      path: '/hub/oc/:id',
      name: 'oc-detail',
      component: () => import('@/views/OcDetailView.vue'),
    },
    {
      path: '/magmas',
      name: 'magmas',
      component: () => import('@/views/MagmasView.vue'),
    },
    {
      // any unknown route also falls back to the hub
      path: '/:pathMatch(.*)*',
      redirect: '/hub',
    },
  ],
})

export default router