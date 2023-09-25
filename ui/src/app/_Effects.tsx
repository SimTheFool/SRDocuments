import { Card } from "@/components/Card";
import { Section } from "@/components/Section";
import { Space } from "@/components/Space";
import { ParagraphStandard } from "@/components/ParagraphStandard";
import { TitleMin } from "@/components/TitleMin";
import { TitleSection } from "@/components/TitleSection";
import { Box } from "@radix-ui/themes";
import { Character } from "resources";
import { TextReplaced } from "@/components/Text";
import { MasonryGrid } from "@/components/MasonryGrid";

type EffectsProps = {
  char: Character;
};

export const Effects = ({ char }: EffectsProps) => {
  return (
    <Section>
      <MasonryGrid compact columns={4}>
        <TitleSection>Effects</TitleSection>
        {char.effects?.map((e) => (
          <Box pr={"2"} pb={"2"}>
            <Card title={e.type}>
              <TitleMin title={e.name} />
              <Space />
              <ParagraphStandard>
                {e.description && <TextReplaced>{e.description}</TextReplaced>}
              </ParagraphStandard>
            </Card>
          </Box>
        ))}
      </MasonryGrid>
    </Section>
  );
};
