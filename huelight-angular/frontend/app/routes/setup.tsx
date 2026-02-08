import type { Route } from "./+types/home";
import { Setup } from "../components/Setup/Setup";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Setup / Connection Configuration" },
    { name: "Configure Hue Bridge" },
  ];
}

export default function Home() {
  return <Setup />;
}
