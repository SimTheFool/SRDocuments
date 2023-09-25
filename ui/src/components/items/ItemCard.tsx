import { capitalize } from "@/utils/capitalize";
import { BaseItem } from "resources";
import { Card } from "../Card";
import { ParagraphStandard } from "../ParagraphStandard";
import { Price } from "../Price";
import { Space } from "../Space";
import { TitleMin } from "../TitleMin";
import { TextReplaced } from "../Text";

type ItemCardProps = {
  children?: React.ReactNode;
  item: BaseItem;
  name: string;
};

export const ItemCard = ({ item, name, children }: ItemCardProps) => {
  return (
    <Card title={item.type} note={`d${item.concealment}`}>
      <TitleMin
        inline
        title={
          <TextReplaced>
            {`${capitalize(name)} ${item.quantity ? `x ${item.quantity}` : ""}`}
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
        {children}
      </ParagraphStandard>
    </Card>
  );
};
