<template>
  <div class="home">
    <p>Gallery dude!</p>
    <b-container class="d-flex flex-row flex-wrap">
      <!-- <div class="d-flex flex-row"> -->
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
          <gallery-card/>
      <!-- </div> -->
    </b-container>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";
import GalleryCard from './components/GalleryCard.vue'

@Component({
  components: {
    GalleryCard,
  },
})
export default class Gallery extends Vue {}
</script>
