<script setup>
import { ref } from 'vue'
import StartButton from './StartButton.vue'
import StopButton from './StopButton.vue'

defineProps({
  isRunning: Boolean
})
const emit = defineEmits(['start', 'stop'])

const countdownActive = ref(false)
const startButtonRef = ref(null)

function handleStart() {
  emit('start')
  countdownActive.value = false
}

function handleStop() {
  if (startButtonRef.value) {
    startButtonRef.value.cancelCountdown()
  }
  emit('stop')
}
</script>

<template>
  <div>
    <StartButton
      ref="startButtonRef"
      :is-running="isRunning"
      @start="handleStart"
      @countdown-started="countdownActive = true"
      @countdown-cancelled="countdownActive = false"
    />
    <StopButton :is-disabled="!isRunning && !countdownActive" @stop="handleStop" />
  </div>
</template>
