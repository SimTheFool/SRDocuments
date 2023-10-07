import { capitalize } from "@/utils/capitalize";
import { Box, Flex } from "@radix-ui/themes";
import { ReactNode } from "react";
import { Companion as CompanionType } from "resources";
import { Card } from "../Card";
import { MasonryGrid } from "../MasonryGrid";
import { ParagraphStandard } from "../ParagraphStandard";
import { Space } from "../Space";
import { TextReplaced } from "../Text";
import { TitleMin } from "../TitleMin";
import { SimpleAction } from "../actions/SimpleAction";
import styles from "./ItemCard.module.css";
import { Slot } from "./Slot";

type CompanionBoxProps = {
  name: string;
  type?: string;
  companion: CompanionType;
  children?: ReactNode;
};

export const CompanionBox = ({
  name,
  type,
  companion,
  children,
}: CompanionBoxProps) => {
  const actions = Object.entries(companion.actions || {}).map(
    ([name, action]) => <SimpleAction name={name} action={action} key={name} />
  );
  const invokSlot = (
    <Slot size="M">puissance - services - vie - maintient</Slot>
  );
  const bottomChildren = [...actions, invokSlot];

  return (
    <Box>
      <Flex className={bottomChildren.length > 0 ? styles.noBorderBottom : ""}>
        <Card title={type}>
          <TitleMin title={<TextReplaced>{capitalize(name)}</TextReplaced>} />
          <ParagraphStandard>
            {companion.description && (
              <>
                <Space />
                <TextReplaced>{companion.description}</TextReplaced>
              </>
            )}
            <Space />
            {children}
            <Space />
            {companion.skills && (
              <>
                <TitleMin
                  title={<TextReplaced>{"Compétences"}</TextReplaced>}
                />
                <Space />
                <Flex>{companion.skills.join(" - ")}</Flex>
                <Space />
              </>
            )}
            {(companion.effects || []).map((effect) => (
              <>
                <Space />
                <TitleMin
                  title={
                    <TextReplaced>{`${capitalize(
                      effect.name || ""
                    )}: `}</TextReplaced>
                  }
                />
                <Space />
                <TextReplaced>{effect.description || ""}</TextReplaced>
              </>
            ))}
          </ParagraphStandard>
        </Card>
      </Flex>

      <MasonryGrid compact columns={1}>
        {bottomChildren.map((child, i) => (
          <Box key={i} className={i == 0 ? "" : styles.bottom}>
            {child}
          </Box>
        ))}
      </MasonryGrid>
    </Box>
  );
};
