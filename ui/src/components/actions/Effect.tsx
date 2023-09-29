import { Box } from "@radix-ui/themes";
import { Effect as EffectType } from "resources";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { Card } from "../Card";

type EffectProps = {
  effect: EffectType;
};

export const Effect = ({ effect }: EffectProps) => {
  return (
    <Box pr={"2"} pb={"2"}>
      <Card title={effect.type}>
        <TitleMin title={effect.name} />
        <Space />
        <ParagraphStandard>
          {effect.description && (
            <TextReplaced>{effect.description}</TextReplaced>
          )}
        </ParagraphStandard>
      </Card>
    </Box>
  );
};
