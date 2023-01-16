<template>
  <RouterView />
</template>

<script setup lang="ts">
import { RouterView } from "vue-router";
</script>
<script lang="ts">
import { defineComponent } from "vue";
export default defineComponent({
  name: "APP",
  mounted() {
    console.log("APP");
    this.$router.replace("/blog-home");
  },
});
</script>
