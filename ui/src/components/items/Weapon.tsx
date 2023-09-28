import { RangeLabels } from "resources";
import { Ruler } from "../Ruler";
import { BaseAction } from "../actions/BaseAction";
import { LoadAction } from "../actions/LoadAction";
import { ShotAction } from "../actions/ShotAction";
import { ItemCard } from "./ItemCard";
import { Weapon as WeaponType } from "resources";

type WeaponProps = { weapon: WeaponType; name: string };

export const Weapon = ({ weapon, name }: WeaponProps) => {
  const { recharger, tir, tir_rafale, tir_semi_auto, ...otherActions } =
    weapon.actions || {};

  return (
    <ItemCard item={weapon} name={name}>
      {{
        inner: (
          <>
            {weapon.range_labels && (
              <DistanceNbRuler distanceByNb={weapon.range_labels} />
            )}
          </>
        ),
        bottom: (
          <>
            {recharger && <LoadAction action={recharger} />}
            {Object.entries(otherActions).map(([name, action]) => (
              <BaseAction name={name} action={action} />
            ))}
            {tir && (
              <ShotAction
                name={"Tir"}
                action={tir}
                rangeLabels={weapon.range_labels}
              />
            )}
            {tir_semi_auto && (
              <ShotAction
                name={"Tir semi"}
                action={tir_semi_auto}
                rangeLabels={weapon.range_labels}
              />
            )}
            {tir_rafale && (
              <ShotAction
                name={"Tir rafale"}
                action={tir_rafale}
                rangeLabels={weapon.range_labels}
              />
            )}
          </>
        ),
      }}
    </ItemCard>
  );
};

const DistanceNbRuler = ({ distanceByNb }: { distanceByNb: RangeLabels }) => {
  const nbs = Object.entries(distanceByNb).map(([key]) => {
    const k = key.replace("r", "");
    const nb = parseInt(k);
    return nb;
  });
  const sortedNbs = nbs.sort();

  return <Ruler items={sortedNbs} />;
};
