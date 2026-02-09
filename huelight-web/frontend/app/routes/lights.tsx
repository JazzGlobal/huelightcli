import type { Route } from "./+types/lights";
import { useLoaderData } from "react-router";
import { getLights, type LightDto } from "../services/api";
import { LightList } from "../components/LightList/LightList";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Lights" },
    { name: "description", content: "Manage your lights" },
  ];
}

export async function loader({}: Route.LoaderArgs) {
  const lights = await getLights();
  return { lights };
}

export default function Lights() {
  const { lights } = useLoaderData() as { lights: LightDto[] };

  return (
    <LightList lightData={lights} />
  );
}
