<script setup>
defineProps({
  isDisabled: Boolean
})
const emit = defineEmits(['stop'])

function handleStop() {
  emit('stop')
}
</script>

<template>
  <button :disabled="isDisabled" @click="handleStop">Arrêter</button>
</template>
