<script setup lang="ts">
import { useMessage } from "naive-ui";
import { setMessageApi } from "../lib/notify";

setMessageApi(useMessage());
</script>
<template></template>
