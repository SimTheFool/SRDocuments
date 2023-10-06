import { Effect as EffectType } from "resources";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";

type EffectProps = {
  effect: EffectType;
};

export const Effect = ({ effect }: EffectProps) => {
  return (
    <Card title={effect.type}>
      <TitleMin title={effect.name} />
      <Space />
      <ParagraphStandard>
        {effect.description && (
          <TextReplaced>{effect.description}</TextReplaced>
        )}
      </ParagraphStandard>
    </Card>
  );
};
