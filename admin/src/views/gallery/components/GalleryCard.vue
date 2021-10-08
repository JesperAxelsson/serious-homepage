<template>
  <div>
    <b-card
      title="Card Title"
      img-src="https://picsum.photos/600/300/?image=25"
      img-alt="Image"
      img-top
      tag="article"
      style="max-height: 15rem; max-width: 15rem"
      class="m-2"
    >
      <b-card-text>
        Some quick example text to build on the card title and make.
      </b-card-text>
    </b-card>
  </div>
</template>

<script lang="ts">
import { Component, Vue } from "vue-property-decorator";

@Component({
  components: {
    // HelloWorld,
  },
})
export default class GalleryCard extends Vue {}
</script>
