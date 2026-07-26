import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      // localhost/ -> redireciona automaticamente para localhost/hub
      path: '/',
      redirect: '/hub',
    },
    {
      path: '/hub',
      name: 'hub',
      component: () => import('@/views/HubView.vue'),
    },
    {
      // qualquer rota desconhecida também cai no hub
      path: '/:pathMatch(.*)*',
      redirect: '/hub',
    },
  ],
})

export default router
