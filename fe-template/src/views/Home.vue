<template>
  <div class="common-layout">
    <el-container>
      <el-header>Header</el-header>
      <el-main>Main</el-main>
      <el-footer>Footer</el-footer>
    </el-container>
  </div>
</template>

<script lang="ts">
import { defineComponent, onMounted } from "vue";
import { ElMessage } from "element-plus";

export default defineComponent({
  name: "Home",
  setup() {
    onMounted(() => {
      if (document.location.protocol === "http:") {
        ElMessage({
          message:
            "==> Home: the protocol is http",
          type: "info",
          duration: 3000,
        });
      }
    });
  },
});
</script>

<style lang="less" scoped>

</style>

