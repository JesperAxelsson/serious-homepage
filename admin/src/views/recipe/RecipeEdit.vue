<template>
  <b-container>
    <p>Title: {{ recipe.title }}</p>
    <p>Description: {{ recipe.description }}</p>
    <div style="max-width: 500px; max-height: 500px">
      <vue-editor v-model="recipe.content" />
    </div>

    <b-button variant="primary" @click="onSave">Save</b-button>
    <b-button variant="primary" @click="onClose">Cancel</b-button>
  </b-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";
import IRecipe from "./recipe-api";

@Component({
  components: {
    VueEditor,
  },
})
export default class RecipeEdit extends Vue {
  @Prop(Object) readonly recipe: IRecipe | undefined;
  content = "";

  onSave() {
    console.log("OnSave edit")
    this.$emit("save", this.recipe);
  }

  onClose() {
    this.$emit("close");
  }
}
</script>
