import { capitalize } from "@/utils/capitalize";
import { Box } from "@radix-ui/themes";
import { Ritual } from "resources";
import { SpellNature } from "../Icons/SpellNature";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { ActionBox } from "./ActionBox";

type RitualActionProps = {
  name: string;
  action: Ritual;
};
export const RitualAction = ({
  name,
  action: { descriptions, nature, type, duration, threshold },
}: RitualActionProps) => {
  const spellSubtitle = [nature && <SpellNature nature={nature} />];

  return (
    <ActionBox title={name} infos={spellSubtitle} type={type}>
      {{
        content: Object.entries(descriptions || {}).map(
          ([key, description]) => (
            <ParagraphStandard>
              {key === "base" ? (
                <TextReplaced>{description}</TextReplaced>
              ) : (
                <>
                  <Space />
                  <Space />
                  <TitleMin
                    title={
                      <TextReplaced>{`${capitalize(key)}: `}</TextReplaced>
                    }
                    inline
                  />
                  <Space />
                  <TextReplaced>{description}</TextReplaced>
                </>
              )}
            </ParagraphStandard>
          )
        ),
        resources: (
          <Box px={"1"}>
            <ParagraphStandard>{`${duration}`}</ParagraphStandard>
            <ParagraphStandard>{`|${threshold}|`}</ParagraphStandard>
          </Box>
        ),
      }}
    </ActionBox>
  );
};
