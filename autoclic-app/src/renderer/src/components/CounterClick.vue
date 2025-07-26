<script setup>
import { ref } from 'vue'

const counter = ref(0)
</script>

<template>
  <div style="height: 100px; background: wheat; user-select: none; color: black" @click="counter++">
    <p>Zone de test :</p>
    <p>
      nombre cliques: {{ counter }}
      <button @click.stop="counter = 0">reset</button>
    </p>
  </div>
</template>
