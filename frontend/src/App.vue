<script setup lang="ts">
import LoginModal from '@/components/auth/LoginModal.vue'
</script>

<template>
  <router-view />
  <!-- Mounted once: any page calls useAuthModal().openLogin() to show it -->
  <LoginModal />
</template>
