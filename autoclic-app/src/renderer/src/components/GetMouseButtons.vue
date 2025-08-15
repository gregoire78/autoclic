<script setup>
import { ref } from 'vue'

const emit = defineEmits(['position'])

const countdown = ref(0)
const isCountingDown = ref(false)
const mousePosition = ref(null)
let intervalId = null

function getMousePosition() {
  window.api.getMousePosition().then((position) => {
    mousePosition.value = position
    emit('position', position) // 🔥 on émet vers le parent
  })
}

function startCountdown() {
  if (isCountingDown.value) return

  countdown.value = 3
  isCountingDown.value = true
  mousePosition.value = null

  intervalId = setInterval(() => {
    countdown.value--
    if (countdown.value <= 0) {
      clearInterval(intervalId)
      intervalId = null
      isCountingDown.value = false
      getMousePosition()
    }
  }, 1000)
}

function resetPosition() {
  mousePosition.value = null
  emit('position', null) // 🔄 on informe aussi le parent du reset
}
</script>

<template>
  <div style="display: flex; gap: 0; align-items: center">
    <button :disabled="isCountingDown" style="min-width: 75%" @click="startCountdown">
      <template v-if="isCountingDown"> Get dans {{ countdown }} sec... </template>
      <template v-else-if="mousePosition">
        🎯 X: {{ mousePosition.x }}, Y: {{ mousePosition.y }}
      </template>
      <template v-else> Recup Souris Coord </template>
    </button>

    <button :disabled="!mousePosition" style="width: 25%" @click="resetPosition">🔄</button>
  </div>
</template>
