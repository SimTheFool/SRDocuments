import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { Sprite as SpriteType } from "resources";
import { MasonryGrid } from "../MasonryGrid";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { StatTable } from "../StatTable";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { BaseAction } from "../actions/BaseAction";
import { Effect } from "../actions/Effect";
import { Card } from "../Card";
import styles from "./ItemCard.module.css";
import { Slot } from "./Slot";

type SpriteProps = {
  name: string;
  sprite: SpriteType;
};

export const Sprite = ({ name, sprite }: SpriteProps) => {
  const actions = Object.entries(sprite.actions || {}).map(([name, action]) => (
    <BaseAction name={name} action={action} />
  ));
  const effects = (sprite.effects || []).map((effect) => (
    <Effect effect={effect} />
  ));
  const invokSlot = (
    <Slot size="M">puissance - services - vie - maintient</Slot>
  );

  const children = [...effects, ...actions, invokSlot];

  const stats = sprite.stats;

  return (
    <Box>
      <Flex className={children.length > 0 ? styles.noBorderBottom : ""}>
        <Card title={"sprite"}>
          <TitleMin
            title={<TextReplaced>{capitalize(name)}</TextReplaced>}
            subtitle={(sprite.skills || []).join(" ")}
          />
          <ParagraphStandard>
            {sprite.description && (
              <>
                <Space />
                <TextReplaced>{sprite.description}</TextReplaced>
              </>
            )}
            <Space />
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
                    [
                      `P+${stats.res}`,
                      `${stats.hit?.base}+P/${stats.hit?.factor}`,
                    ],
                  ]}
                />
              </>
            )}
          </ParagraphStandard>
        </Card>
      </Flex>

      <MasonryGrid compact columns={1}>
        {children.map((child, i) => (
          <Box className={i == 0 ? "" : styles.bottom}>{child}</Box>
        ))}
      </MasonryGrid>
    </Box>
  );
};
