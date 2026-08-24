<script setup lang="ts">
    import LoginModal from '@/components/auth/LoginModal.vue'
    import { onMounted, ref } from 'vue'
    import { useUserStore } from '@/stores/user'
    const userStore = useUserStore()
    const isBootstrapping = ref(true)

    onMounted(async () => {
        await userStore.refresh()
        isBootstrapping.value = false
    })
</script>

<template>
  <router-view />
  <!-- Mounted once: any page calls useAuthModal().openLogin() to show it -->
  <LoginModal />
</template>
