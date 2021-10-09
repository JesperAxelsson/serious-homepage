<template>
  <b-container>
    <h3 v-if="recipe.id > 0">Editing {{ recipe.title }}</h3>
    <h3 v-else>Create new recipe!</h3>
    <b-button variant="primary" class="m-1" @click="onSave"> Save </b-button>
    <b-button variant="primary" class="m-1" @click="onClose">Cancel</b-button>
    <b-button v-if="recipe.id > 0" variant="danger" class="m-1" @click="onDelete">Delete</b-button>

    <b-row class="my-1">
      <b-col sm="2">
        <label for="input-title">Title:</label>
      </b-col>
      <b-col sm="10">
        <b-form-input
          id="input-title"
          size="sm"
          placeholder="Title"
          v-model="recipe.title"
        />
      </b-col>
    </b-row>

    <b-row class="my-1">
      <b-col sm="2">
        <label for="input-description">Description:</label>
      </b-col>
      <b-col sm="10">
        <b-form-input
          id="input-description"
          size="sm"
          placeholder="Description"
          v-model="recipe.description"
        />
      </b-col>
    </b-row>

    <!-- <div style="max-width: 500px"> -->

    <vue-editor v-model="recipe.content" />
    <!-- </div> -->
  </b-container>
</template>

<script lang="ts">
import { Component, Prop, Vue } from "vue-property-decorator";
import { VueEditor } from "vue2-editor";
import { IRecipe } from "./recipe-api";

@Component({
  components: {
    VueEditor,
  },
})
export default class RecipeEdit extends Vue {
  @Prop(Object) readonly recipe: IRecipe | undefined;
  content = "";

  onSave() {
    console.log("OnSave edit");
    this.$emit("save", this.recipe);
  }

  onDelete() {
    console.log("onDelete edit");
    this.$emit("delete", this.recipe);
  }

  onClose() {
    this.$emit("close");
  }
}
</script>
