import Theme from "vitepress/theme";
import "./styles.css";
import RoadmapCard from "./components/RoadmapCard.vue";
import RoadmapSection from "./components/RoadmapSection.vue";

export default {
  extends: Theme,
  enhanceApp({ app }) {
    app.component("RoadmapCard", RoadmapCard);
    app.component("RoadmapSection", RoadmapSection);
  },
};
