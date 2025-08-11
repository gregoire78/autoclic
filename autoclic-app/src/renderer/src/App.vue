<script setup>
import { ref, computed, onMounted } from 'vue'
import ControlButtons from './components/ControlButtons.vue'

const cps = ref()
const isRunning = ref(false)

const containerStyle = computed(() => ({
  backgroundColor: isRunning.value ? 'orange' : 'var(--color-background)',
  padding: '10px',
  height: '100%'
}))

onMounted(() => {
  window.api.receive('running', (value) => {
    isRunning.value = value
  })
  window.api.getCps().then((value) => {
    cps.value = value
  })
})

function updateCps() {
  window.api.setCps(cps.value)
}

function handleStart() {
  window.electron.ipcRenderer.send('start')
}

function handleStop() {
  window.electron.ipcRenderer.send('stop')
}
</script>

<template>
  <div :style="containerStyle">
    <div style="margin-bottom: 5px; font-size: 10px">
      Démarrer :
      <kbd>Ctrl</kbd> + <kbd>Alt</kbd> + <kbd>A</kbd> Stop : <kbd>Q</kbd>
    </div>
    <div style="margin-bottom: 5px">
      cps :
      <input
        v-model.number="cps"
        style="max-width: 30px"
        type="number"
        min="0"
        @input="updateCps"
      />
      ms
    </div>
    <ControlButtons :is-running="isRunning" @start="handleStart" @stop="handleStop" />
  </div>
</template>
