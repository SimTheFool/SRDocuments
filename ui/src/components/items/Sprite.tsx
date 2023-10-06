import { Sprite as SpriteType } from "resources";
import { StatTable } from "../StatTable";
import { CompanionBox } from "./CompanionBox";

type SpriteProps = {
  name: string;
  sprite: SpriteType;
};

export const Sprite = ({ name, sprite }: SpriteProps) => {
  const stats = sprite.stats;

  return (
    <CompanionBox companion={sprite} name={name} type={"sprite"}>
      {stats && (
        <>
          <StatTable
            compact
            items={[
              ["Firew.", "Trait.", "Corr.", "Att."],
              [
                `P+${stats.firewall}`,
                `P+${stats.traitement}`,
                `P+${stats.corruption}`,
                `P+${stats.attaque}`,
              ],
            ]}
          />
          <StatTable
            compact
            items={[
              ["RES", "Vie"],
              [`P+${stats.res}`, `${stats.hit?.base}+P/${stats.hit?.factor}`],
            ]}
          />
        </>
      )}
    </CompanionBox>
  );
};
