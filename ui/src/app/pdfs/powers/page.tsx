import { Card } from "@/components/Card";
import { MasonryGrid } from "@/components/MasonryGrid";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { Space } from "@/components/Space";
import { StatTable } from "@/components/StatTable";
import { TextReplaced } from "@/components/Text";
import { TitleMin } from "@/components/TitleMin";
import { TitleSection } from "@/components/TitleSection";
import { BaseAction } from "@/components/actions/BaseAction";
import { Effect } from "@/components/actions/Effect";
import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { Sprite as SpriteType, characters } from "resources";
import { A4Format } from "../A4Format";
import { Sprite } from "@/components/items/Sprite";

const shrimp = characters.shrimp;

export default function Home() {
  return (
    <A4Format border>
      <Box pt={"2"}>
        <MasonryGrid columns={3}>
          <Box>
            <TitleSection>Pouvoirs et companions</TitleSection>
            <Space />
          </Box>
          {Object.entries(shrimp.sprites || {}).map(([name, sprite]) => {
            return <Sprite name={name} sprite={sprite} />;
          })}
        </MasonryGrid>
      </Box>
    </A4Format>
  );
}
