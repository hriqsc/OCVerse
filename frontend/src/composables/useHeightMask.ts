import { ref, computed } from 'vue'

export function useHeightMask(initialRaw = '') {
  const raw = ref(initialRaw.replace(/\D/g, '').slice(0, 3))

  const display = computed(() => {
    if (!raw.value) return ''
    if (raw.value.length === 1) return raw.value
    return `${raw.value[0]},${raw.value.slice(1)} m`
  })

  function onInput(event: Event) {
    const input = event.target as HTMLInputElement
    const digits = input.value.replace(/\D/g, '').slice(0, 3)
    raw.value = digits
    input.value = display.value
  }

  function reset(value: string) {
    raw.value = value.replace(/\D/g, '').slice(0, 3)
  }

  return { raw, display, onInput, reset }
}