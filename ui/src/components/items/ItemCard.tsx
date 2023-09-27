import { capitalize } from "@/utils/capitalize";
import { BaseItem } from "resources";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Price } from "../Price";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { TextReplaced } from "../Text";
import { Box, Flex } from "@radix-ui/themes";
import { MasonryGrid } from "../MasonryGrid";
import styles from "./ItemCard.module.css";
import React from "react";

type ItemCardProps = {
  children?: {
    aside?: React.ReactNode;
    bottom?: React.ReactElement;
    inner?: React.ReactNode;
  };
  item: BaseItem;
  name: string;
};

export const ItemCard = ({ item, name, children }: ItemCardProps) => {
  const bottomItemNb = React.Children.toArray(
    children?.bottom?.props.children
  ).filter((x) => x).length;

  return (
    <Box>
      <Flex className={bottomItemNb > 0 ? styles.noBorderBottom : ""}>
        <Card title={item.type} note={`d${item.concealment}`}>
          <TitleMin
            inline
            title={
              <TextReplaced>
                {`${capitalize(name)} ${
                  item.quantity ? `x ${item.quantity}` : ""
                }`}
              </TextReplaced>
            }
            subtitle={item.manufacturer}
          />
          <TitleMin
            subtitle={
              <>
                <Price price={item.price} />
                <Space inline />
                {!item.legal ? "Illégal" : item.licenced ? "Licencié" : ""}
              </>
            }
          />
          <ParagraphStandard>
            <Space />
            {item.description}
            <Space />
            {children?.inner}
          </ParagraphStandard>
        </Card>
      </Flex>

      <MasonryGrid compact columns={1}>
        {React.Children.toArray(children?.bottom?.props.children).map(
          (child, i) => (
            <Box className={i == 0 ? "" : styles.bottom}>{child}</Box>
          )
        )}
      </MasonryGrid>
    </Box>
  );
};
