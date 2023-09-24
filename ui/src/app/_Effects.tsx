import { Card } from "@/components/Card";
import { Section } from "@/components/Section";
import { Space } from "@/components/Space";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { TitleMin } from "@/components/TitleMin";
import { TitleSection } from "@/components/TitleSection";
import { Box } from "@radix-ui/themes";
import { Character } from "resources";
import { TextWithIcons } from "@/components/Text";
import { MasonryGrid } from "@/components/MasonryGrid";

type EffectsProps = {
  char: Character;
};

export const Effects = ({ char }: EffectsProps) => {
  return (
    <Section>
      <MasonryGrid columns={4}>
        <Box pb={"1"}>
          <TitleSection>Effects</TitleSection>
        </Box>
        {char.effects?.map((e) => (
          <Box pr={"2"} pb={"2"}>
            <Card title={e.type}>
              <TitleMin title={e.name} />
              <Space />
              <ParagraphStandard>
                {e.description && (
                  <TextWithIcons>{e.description}</TextWithIcons>
                )}
              </ParagraphStandard>
            </Card>
          </Box>
        ))}
      </MasonryGrid>
    </Section>
  );
};
