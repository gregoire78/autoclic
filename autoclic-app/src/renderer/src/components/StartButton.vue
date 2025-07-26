<script setup>
import { ref } from 'vue'

const props = defineProps({
  isRunning: Boolean
})
const emit = defineEmits(['start', 'countdown-started', 'countdown-cancelled'])

const countdown = ref(0)
const isCountingDown = ref(false)
let intervalId = null

function startCountdown() {
  if (isCountingDown.value || props.isRunning) return

  countdown.value = 5
  isCountingDown.value = true
  emit('countdown-started')

  intervalId = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      clearInterval(intervalId)
      intervalId = null
      isCountingDown.value = false
      emit('start')
    }
  }, 1000)
}

function cancelCountdown() {
  if (intervalId) {
    clearInterval(intervalId)
    intervalId = null
    isCountingDown.value = false
    countdown.value = 0
    emit('countdown-cancelled')
  }
}

defineExpose({ cancelCountdown })
</script>

<template>
  <button :disabled="isCountingDown || isRunning" @click="startCountdown">
    {{ isCountingDown ? `Démarrage dans ${countdown} sec...` : 'Démarrer' }}
  </button>
</template>
