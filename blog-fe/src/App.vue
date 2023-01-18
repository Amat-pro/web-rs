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
    console.log("todo check token");
    let token_valid = true;
    if (token_valid) {
      this.$router.replace("/blog-home");
    } else {
      this.$router.replace("/article");
    }
  },
});
</script>
